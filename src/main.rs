use std::{io::{BufRead, BufReader, Write},net::{TcpListener, TcpStream}, thread};
#[allow(unused_imports)]

fn main() {
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream{
            Ok(stream)=> {
                thread::spawn(|| {
                    handle_client(stream);
                });
            },
            Err(e)=> {
                eprintln!("Failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream){
    println!("accepted new connection");

    let buf_reader = BufReader::new(&stream);
    let mut lines = buf_reader.lines();
    let request_line = lines.next().unwrap().unwrap();
    let uri = request_line.split_whitespace().nth(1).unwrap();

    let resp_200 = "HTTP/1.1 200 OK";
    let resp_404 = "HTTP/1.1 404 Not Found";

    let (status_line, body) = if uri == "/" {
        (resp_200, "".to_string())
    } else if uri.starts_with("/echo") {
        let echo = uri.split("/").nth(2).unwrap_or("");
        (resp_200, echo.to_string())
    } else if uri.starts_with("/user-agent") {
        let mut user_agent = String::new();
        for line in lines {
            let line = line.unwrap();
            if line.is_empty() {
                break;
            }
            if line.starts_with("User-Agent") {
                user_agent = line.split_once(": ").unwrap_or(("", "")).1.to_string();
            }
        }
        (resp_200, user_agent.clone())
    } else {
        (resp_404, "".to_string())
    };
    
    let response = format!(
        "{status_line}\r\nContent-Type: text/plain\r\nContent-Length: {len}\r\n\r\n{user_agent}",
        len=body.len(),
        user_agent=body.clone()
    );
    stream.write_all(response.as_bytes()).unwrap();
}
