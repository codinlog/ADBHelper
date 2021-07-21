use crate::protocol::cmd;
use crate::protocol::Message;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::ops::Add;

struct Command {
    local_id: Cell<u32>,
    cons: RefCell<HashMap<u32, TcpStream>>,
}

impl Command {
    pub fn new() -> Command {
        Command {
            local_id: Cell::new(0),
            cons: RefCell::new(HashMap::new()),
        }
    }
    pub fn connect(&self, addr: impl ToSocketAddrs) -> Result<u32, Error> {
        let mut con = TcpStream::connect(addr)?;
        con.write(&Message::encode(Message::cnnx_msg()))?;
        let mut rtn = Message::decode(&mut con);
        if rtn.cmd == cmd::A_AUTH {
            let (msg, pk) = Message::sign_msg(rtn);
            con.write(&Message::encode(msg))?;
            rtn = Message::decode(&mut con);
            if rtn.cmd == cmd::A_AUTH {
                con.write(&Message::encode(Message::pkey_msg(pk)))?;
            }
            rtn = Message::decode(&mut con);
        }
        if rtn.cmd != cmd::A_CNXN {
            return Err(Error::new(
                ErrorKind::NotConnected,
                "Not connected successful,rtn cmd not equal cnxn",
            ));
        }
        let local_id = self.local_id.get().add(1);
        self.local_id.set(local_id);
        self.cons.borrow_mut().insert(local_id, con);
        Ok(self.local_id.get())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1() {
        let cmd = Command::new();
        let local_id = cmd
            .connect("192.168.31.112:5555")
            .expect("Err Connect Remote");
        println!("local id: {}", local_id);
        let mut hash = cmd.cons.borrow_mut();
        let mut tcp = hash.get_mut(&local_id).unwrap();
        let msg = Message::new(cmd::A_OPEN, 100, 0, b"shell:input keyevent 4\0".to_vec());
        let msg = Message::encode(msg);
        tcp.write(msg.as_ref()).unwrap();
        let msg = Message::decode(&mut tcp);
        let msg = Message::decode(&mut tcp);
        let hi = String::from_utf8_lossy(&msg.payload);
    }
}
