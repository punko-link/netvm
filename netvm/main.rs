// mod core;
// mod io;
// mod dist;
//
// fn main() {
//     let mut stack = core::Stack::new(dist::stack());
//     let mut prog = dist::prog();
//
//     loop {
//         let result = stack.run(prog);
//         if result.return_type == core::ReturnType::Exit {
//             break;
//         }
//
//         stack = result.stack;
//         prog = result.prog;
//     }
// }




use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream, Shutdown},
};

mod master;
mod io;
mod stdlib;


fn main() {
    let mut root = master::Master::new();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();


    for stream in listener.incoming() {
        let stream = stream.unwrap();

        root = io::tcp::handle_connection(stream, root);
    }
}



