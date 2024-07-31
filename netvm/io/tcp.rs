use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream, Shutdown},
};
use master::Master;
use stdlib::interface;

pub fn handle_connection(mut stream: TcpStream, mut root: Master) -> Master {
    let buf_reader = BufReader::new(&mut stream);
    let mut incoming_data: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();


    let head_str = incoming_data.remove(0);
    let head_vec = head_str.split(' ');
    let mut method = "";
    let mut path = "";

    for one in head_vec {
        if method.len() == 0 { method = one }
        else if path.len() == 0 { path = one };
    }


    match method {
        "GET" => {
            stream.write_all("HTTP/1.1 200 OK\r\n\r\nHallo\r\n\r\n".as_bytes()).unwrap();
        },
        "PUNKO" => {
            let incoming_package = interface::Incoming { to: path.to_string(), data: Vec::from(incoming_data.remove(0)) };
            let mut master_result = interface::Outgoing::default();
            (master_result, root) = root.run(incoming_package);

            if !master_result.is_success {
                stream.write_all(format!("FAILED\r\n\r\n{}\r\n\r\n", String::from_utf8(master_result.data).unwrap()).as_bytes());
            }
            else {
                stream.write_all(format!("SUCCESS\r\n\r\n{}\r\n\r\n", String::from_utf8(master_result.data).unwrap()).as_bytes());
            }
        },
        _ => {
            stream.write_all("HTTP/1.1 400 Method is not allowed. Allowed are GET, PUNKO \r\n\r\n".as_bytes()).unwrap();
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }

    root
}
