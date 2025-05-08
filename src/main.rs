#[allow(unused_imports)]
use std::io::BufRead;
use std::io::BufReader;
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

    let uri = request_line.split_whitespace().nth(1).unwrap();
    let status_line = if uri == "/" {
        "HTTP/1.1 200 OK"
    } else if uri.starts_with("/echo") {
        "HTTP/1.1 200 OK"
    } else {
        "HTTP/1.1 404 Not Found"
    };

    let request_uri: &str = request_line.split_whitespace().nth(1).unwrap();
    let uri_params: &str = request_uri.split("/").nth(2).unwrap_or("");

    let length = uri_params.len();

    let response =
        format!("{status_line}\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{uri_params}");

    stream.write_all(response.as_bytes()).unwrap();
}
