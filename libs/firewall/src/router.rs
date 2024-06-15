mod route;

use std::net::TcpStream;
use crate::Firewall;

impl Firewall {
    pub fn router(self, path: String, data: Vec<u8>, mut stream: TcpStream) -> Firewall {
        println!("{path}");
        println!("{}", String::from_utf8(data).unwrap());

        self
    }

}
