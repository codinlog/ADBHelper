pub mod debug {
    use crate::protocol::cmd;
    pub fn cmd_type_is(t: u32) {
        match t {
            cmd::A_SYNC => {
                println!("A_SYNC");
            }
            cmd::A_CLSE => {
                println!("A_CLSE");
            }
            cmd::A_CNXN => {
                println!("A_CNXN");
            }
            cmd::A_OKAY => {
                println!("A_OKAY");
            }
            cmd::A_OPEN => {
                println!("A_OPEN");
            }
            cmd::A_WRTE => {
                println!("A_WRTE");
            }
            cmd::A_AUTH => {
                println!("A_AUTH");
            }
            _ => {
                println!("unknown cmd");
            }
        }
    }
}
