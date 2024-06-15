use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use crate::Firewall;


impl Firewall {

    pub fn handle_connection(self, mut stream: TcpStream) -> Firewall {

        let buf_reader = BufReader::new(&mut stream);
        let mut incoming: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take(65535)
            .collect();

        let head = incoming.remove(0);
        let mut path = String::new();
        let mut i = 0;
        for p in head.split(" ") {
            if i == 1 {
                path = String::from(p);
                break
            }
            i += 1;
        }

        let data = Vec::from(incoming.get(incoming.len() - 1).unwrap().clone());

        self.router(path, data, stream)
    }
}
