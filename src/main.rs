#[allow(unused_imports)]
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream){
    println!("accepted new connection");
    let resp = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(resp.as_bytes()).unwrap();
}
