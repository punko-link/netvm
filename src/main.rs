use std::io::{BufRead, Read, Write};
use std::net::{TcpListener, TcpStream};
use firewall::Firewall;

mod master;

fn main() {
    let mut firewall = Firewall::new();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        firewall = firewall.handle_connection(stream);
    }

}


