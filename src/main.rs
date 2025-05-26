mod file_handler;
mod request;
mod response;
mod routers;
mod utils;

use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    result::Result::{Err, Ok},
    thread,
};

use crate::request::Request;
use crate::routers::handle_routes;
#[allow(unused_imports)]

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("Failed: {}", e),
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    println!("accepted new connection");
    let mut stream = stream;
    loop {
        let req: Request = {
            let stream2 = &mut stream;
            Request::new(stream2)
        };
        let conn_closed = req.should_close_conn();
        let response = handle_routes(req);

        stream
            .write_all(format!("{response}").as_bytes())
            .unwrap_or_default();
        (!response.compressed_body.is_none()).then(|| {
            stream
                .write_all(&response.compressed_body.unwrap())
                .unwrap()
        });

        if conn_closed {
            break;
        }

        stream.flush().unwrap()
    }
}
