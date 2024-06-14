use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

mod master;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }

}

fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let incoming: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take(65535)
        .collect();
}
