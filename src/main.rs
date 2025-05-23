mod request;
mod response;
mod file_handler;
mod routers;
mod utils;

use std::{
    io::Write,
    net::{TcpListener, TcpStream}, 
    thread,
};
use crate::request::Request;
use crate::routers::handle_routes;
#[allow(unused_imports)]


fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream{
            Ok(stream)=> {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            },
            Err(e)=> {
                eprintln!("Failed: {}", e);
            }
        }
    }
}

fn handle_connection(stream: TcpStream){
    println!("accepted new connection");
    let mut stream = stream;
    let req: Request = {
        let stream2 = &mut stream;
        Request::new(stream2)
    };
    let response = handle_routes(req);
    
    stream.write_all(format!("{response}").as_bytes()).unwrap();
    (!response.compressed_body.is_none())
        .then(|| stream.write_all(&response.compressed_body.unwrap()).unwrap());
    stream.flush().unwrap()
}
