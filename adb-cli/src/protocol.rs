#![allow(unused)]

use crate::crypto;
use async_std::fs::read;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use bytes::buf;
use bytes::BufMut;
use bytes::BytesMut;
use std::convert::TryFrom;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::ops::RangeFrom;
use std::usize;

use crate::util::debug;
use crate::protocol::arg0::VERSION;
use crate::protocol::arg1::MAX_PAYLOAD;

pub mod cmd {
    pub const A_SYNC: u32 = 0x434e5953;
    pub const A_CNXN: u32 = 0x4e584e43;
    pub const A_OPEN: u32 = 0x4e45504f;
    pub const A_OKAY: u32 = 0x59414b4f;
    pub const A_CLSE: u32 = 0x45534c43;
    pub const A_WRTE: u32 = 0x45545257;
    pub const A_AUTH: u32 = 0x48545541;
}

pub mod arg0 {
    pub const VERSION: u32 = 0x01000000;
    pub const AUTH_TOKEN: u32 = 1;
    pub const AUTH_SIGNATURE: u32 = 2;
    pub const AUTH_PUBLICKEY: u32 = 3;
}

pub mod arg1 {
    pub const MAX_PAYLOAD: u32 = 4096;
}

const MESSAGE_HEADER_LENGTH: usize = 24;
const CONNECT_PAYLOAD: &[u8; 13] = b"host::notadb\0";

#[derive(Debug)]
pub struct Message {
    pub cmd: u32,
    pub arg0: u32,
    pub arg1: u32,
    pub length: u32,
    pub checksum: u32,
    pub magic: u32,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new(cmd: u32, arg0: u32, arg1: u32, payload: Vec<u8>) -> Message {
        Message {
            cmd,
            arg0,
            arg1,
            length: payload.len() as u32,
            checksum: Self::payload_checksum(&payload),
            magic: cmd ^ 0xFFFFFFFF, //*!cmd*/
            payload,
        }
    }
    pub fn encode(msg: Message) -> Vec<u8> {
        let mut buff = vec![];
        buff.write_u32::<LittleEndian>(msg.cmd);
        buff.write_u32::<LittleEndian>(msg.arg0);
        buff.write_u32::<LittleEndian>(msg.arg1);
        buff.write_u32::<LittleEndian>(msg.length);
        buff.write_u32::<LittleEndian>(msg.checksum);
        buff.write_u32::<LittleEndian>(msg.magic);
        buff.put_slice(&msg.payload);
        buff
    }
    pub fn decode<R>(read: &mut R) -> Self
        where
            R: Read,
    {
        let mut header = [0; 24];
        read.read_exact(&mut header);
        let mut cus = Cursor::new(header);
        let rtn_cmd = cus.read_u32::<LittleEndian>().expect("Err Rtn Cmd");
        let rtn_arg0 = cus.read_u32::<LittleEndian>().expect("Err Rtn Arg0");
        let rtn_arg1 = cus.read_u32::<LittleEndian>().expect("Err Rtn Arg1");
        let rtn_length = cus.read_u32::<LittleEndian>().expect("Err Rtn Length");
        let rtn_checksum = cus.read_u32::<LittleEndian>().expect("Err Rtn Checksum");
        let rtn_magic = cus.read_u32::<LittleEndian>().expect("Err Rtn Magic");
        let mut rtn_payload = vec![0; rtn_length as usize];
        read.read_exact(&mut rtn_payload);
        println!(
            "{}",
            format!(
                "rtn_cmd:{:4},
                rtn_arg0:{:4},
                rtn_arg1:{:4},
                rtn_length:{:4},
                rtn_checksum:{:4},
                rtn_magic:{},
                rtn_payload:{:?}",
                rtn_cmd, rtn_arg0, rtn_arg1, rtn_length, rtn_checksum, rtn_magic, rtn_payload
            )
        );
        debug::cmd_type_is(rtn_cmd);
        Message {
            cmd: rtn_cmd,
            arg0: rtn_arg0,
            arg1: rtn_arg1,
            length: rtn_length,
            checksum: rtn_checksum,
            magic: rtn_magic,
            payload: rtn_payload,
        }
    }
    pub fn payload_checksum(payload: &[u8]) -> u32 {
        let mut checksum = 0_u32;
        payload.iter().for_each(|&b| {
            checksum += b as u32;
        });
        checksum
    }
    pub fn is_validate_message(msg: &Message) -> bool {
        if msg.cmd != (msg.magic ^ 0xFFFFFFFF) {
            return false;
        }
        if msg.length != 0 {
            if Self::payload_checksum(&msg.payload) != msg.checksum {
                return false;
            }
        }
        true
    }

    pub fn cnnx_msg() -> Message {
        Message::new(cmd::A_CNXN, VERSION, MAX_PAYLOAD, CONNECT_PAYLOAD.to_vec())
    }
    pub fn sign_msg(mut msg: Message) -> (Message, Vec<u8>) {
        let (payload, mut pk) = crypto::sign_data(&msg.payload);
        pk.push(b'\0');
        msg.cmd = cmd::A_AUTH;
        msg.arg0 = arg0::AUTH_SIGNATURE;
        msg.arg1 = 0;
        msg.length = payload.len() as u32;
        msg.checksum = Message::payload_checksum(&payload);
        msg.magic = !msg.cmd;
        msg.payload = payload;
        (msg, pk)
    }
    pub fn pkey_msg(pk: Vec<u8>) -> Message {
        Message::new(cmd::A_AUTH, arg0::AUTH_PUBLICKEY, 0, pk)
    }
}

#[cfg(test)]
mod test {
    use super::arg0::VERSION;
    use super::arg1::MAX_PAYLOAD;
    use super::cmd;
    use super::Message;
    use super::CONNECT_PAYLOAD;
    use super::MESSAGE_HEADER_LENGTH;
    use crate::crypto::*;
    use crate::protocol::arg0;
    use byteorder::ByteOrder;
    use byteorder::LittleEndian;
    use byteorder::ReadBytesExt;
    use bytes::buf;
    use std::convert::TryFrom;
    use std::fs::write;
    use std::fs::OpenOptions;
    use std::io::Cursor;
    use std::net::TcpStream;
    use std::path::Path;
    use std::{
        io::{Read, Write},
        str,
    };

    #[test]
    fn t1() {
        //CNXX
        let msg = Message::new(cmd::A_CNXN, VERSION, MAX_PAYLOAD, CONNECT_PAYLOAD.to_vec());
        let mut stream = TcpStream::connect("192.168.31.112:5555").unwrap();
        let msg = Message::encode(msg);
        stream.write(msg.as_ref()).unwrap();
        let msg = Message::decode(&mut stream);
        //Sign
        let (msg, mut pk) = Message::sign_msg(msg);
        let msg = Message::encode(msg);
        stream.write(msg.as_ref()).unwrap();
        let msg = Message::decode(&mut stream);
        //Public Key
        println!("pk is:{:?}", pk);
        let msg = Message::new(cmd::A_AUTH, arg0::AUTH_PUBLICKEY, 0, pk);
        let msg = Message::encode(msg);
        stream.write(msg.as_ref()).unwrap();
        let msg = Message::decode(&mut stream);
        //open
        let msg = Message::new(cmd::A_OPEN, 1, 0, b"shell:input keyevent 4\0".to_vec());
        let msg = Message::encode(msg);
        stream.write(msg.as_ref()).unwrap();
        let msg = Message::decode(&mut stream);
        let msg = Message::decode(&mut stream);
        let hi = String::from_utf8_lossy(&msg.payload);
        println!("{}", hi);
    }
}
