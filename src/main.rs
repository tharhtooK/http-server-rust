use std::{
    collections::HashMap, fs, io::{BufRead, BufReader, Error, Read, Write}, net::{TcpListener, TcpStream}, thread
};
#[allow(unused_imports)]

#[derive(Debug)]
struct Request {
    uri: String,
    http_method: String,
    user_agent: Option<String>,
    body: String,
    file_name: String,
}

impl Request {
    fn get_http_body(buf_reader: &mut BufReader<&mut TcpStream>, request_line: &String) -> String {
        let mut size = 0;
        let linesplit = request_line.split("\n")
                                                .filter_map(|l| 
                                                    l.split_once(":")
                                                    .map(|l| (l.0, l.1.trim()))
                                                    .filter(|l| l.0.starts_with("Content-Length"))
                                                );
        // for l in linesplit {
        //     if l.starts_with("Content-Length") {
        //         let sizeplit = l.split(":");
        //         for s in sizeplit {
        //             if !(s.starts_with("Content-Length")) {
        //                 size = s.trim().parse::<usize>().unwrap();
        //             }
        //         }
        //     }
        // }
        let mut buffer = vec![0; size]; 
        buf_reader.read_exact(&mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    fn get_uri(request_line: &String) -> &str {
        request_line.split_whitespace().nth(1).unwrap()
    }

    fn get_http_method(request_line: &String) -> &str {
        request_line.split_whitespace().nth(0).unwrap()
    }

    fn get_header_map(request_line: &String) -> HashMap<String, String> {
        let mut request_map: HashMap<String, String> = HashMap::new();
        let res: Vec<_> = request_line.split("\n")
                        .filter_map(|l|                         
                            Some(l.split_once(":").map(|l| (l.0.to_string(), l.1.trim().to_string())))
                            .take_if(|l| !l.is_none())
                        )
                        .collect();
        for r in res {
            let r = r.unwrap();
            request_map.insert(r.0, r.1);
        }
        request_map
    }

    fn get_file_name(uri: &String) -> String {
        uri.split("/").nth(2).unwrap_or("").to_string()
    }

    fn new(stream: &mut TcpStream) -> Self {
        let mut buf_reader = BufReader::new(stream);
        
        let mut request_line = String::new();
        loop {
            let r = buf_reader.read_line(&mut request_line).unwrap();
            if r < 3 {
                break;
            }
        }
        let uri = Self::get_uri(&request_line).to_string();
        let file_name = Self::get_file_name(&uri);
        let http_method = Self::get_http_method(&request_line).to_string();
        let body = Self::get_http_body(&mut buf_reader, &request_line).to_string();
        let headers_map = Self::get_header_map(&request_line);
        let user_agent = headers_map.get("User-Agent").map(|v| v.to_string());

        Self {
            uri: uri,
            file_name: file_name,
            http_method: http_method,
            user_agent: user_agent,
            body: body,
        }
    }
}

struct Response {
    status_line: String,
    content_type: String,
    body: String,
}

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

fn read_file(file_name: &str) -> Result<String, Error> {
    let file_path = format!("/tmp/data/codecrafters.io/http-server-tester/{file_name}");

    fs::read_to_string(file_path)
}

fn create_file(file_name: &str, content: String) -> Result<(), Error> {
    let file_path = format!("/tmp/data/codecrafters.io/http-server-tester/{file_name}");
    let file = fs::File::create(file_path);
    file.expect("File not found!").write_all(content.as_bytes())
}

fn handle_client(mut stream: TcpStream){
    println!("accepted new connection");
    let mut stream = stream;
    let req: Request;
    {
        let stream2 = &mut stream;
        req = Request::new(stream2);
    }

    let resp_200 = "HTTP/1.1 200 OK";
    let resp_201 = "HTTP/1.1 201 Created";
    let resp_404 = "HTTP/1.1 404 Not Found";
    let content_type_text = "text/plain";
    let content_type_octet = "application/octet-stream";

    let (status_line, content_type, body) = if req.uri == "/" {
        (resp_200, content_type_text, "".to_string())
    } else if req.uri.starts_with("/echo") {
        let echo = req.uri.split("/").nth(2).unwrap_or("");
        (resp_200, content_type_text, echo.to_string())
    } else if req.uri.starts_with("/user-agent") {
        (resp_200, content_type_text, req.user_agent.unwrap())
    } else if req.uri.starts_with("/files") {
        match req.http_method.as_str() {
            "POST" => {
                match create_file(&req.file_name, req.body) {
                    Ok(_) => (resp_201, "", "".to_string()),
                    Err(_) => (resp_404, "", "".to_string())
                }
            },
            _ => {
                match read_file(&req.file_name) {
                    Ok(content) => {
                        (resp_200, content_type_octet, content)
                    },
                    Err(_) => {
                        (resp_404, content_type_text, "".to_string())
                    }
                }
            }
        }
    } else {
        (resp_404, content_type_text, "".to_string())
    };

    println!("status: {status_line}, content_type: {content_type}, body: {body}");
    
    let response = format!(
        "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {len}\r\n\r\n{body}",
        len=body.len(),
        content_type=content_type,
        body=body.clone()
    );
    
    stream.write_all(response.as_bytes()).unwrap();
    
}
