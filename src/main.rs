use std::io::BufRead;
use std::io::BufReader;
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
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let status_line = if request_line == "GET / HTTP/1.1" {
        "HTTP/1.1 200 OK"
    } else {
        "HTTP/1.1 404 Not Found"
    };

    let response = format!("{status_line}\r\n\r\n");

    stream.write_all(response.as_bytes()).unwrap();
    
}
