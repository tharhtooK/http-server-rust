use std::{
    collections::HashMap, fmt::{self, format, write}, fs, io::{BufRead, BufReader, Error, Read, Write}, net::{TcpListener, TcpStream}, thread
};
#[allow(unused_imports)]

enum HttpCode {
    OK,
    Created,
    NotFound,
}

impl fmt::Display for HttpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match &self {
            Self::OK => "HTTP/1.1 200 OK".to_string(),
            Self::Created => "HTTP/1.1 201 Created".to_string(),
            Self::NotFound => "HTTP/1.1 404 Not Found".to_string(),
        })
    }
}

enum ContentType {
    None,
    Text,
    Octet
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match &self {
            Self::None => "".to_string(),
            Self::Text => "text/plain".to_string(),
            Self::Octet => "application/octet-stream".to_string(),
        })
    }
}

#[derive(Debug)]
struct Request {
    uri: String,
    http_method: String,
    encoding: Vec<String>,
    user_agent: Option<String>,
    echo_path: String,
    body: String,
    file_name: String,
}

impl Request {
    fn get_http_body(buf_reader: &mut BufReader<&mut TcpStream>, request_line: &String) -> String {
        let mut size = 0;
        let _: Vec<_> = request_line.split("\n")
                        .filter_map(|l| 
                            l.split_once(":")
                            .filter(|l| l.0.starts_with("Content-Length"))
                            .map(|l| size = l.1.trim().parse::<usize>().unwrap())
                        ).collect();

        let mut buffer = vec![0; size]; 
        buf_reader.read_exact(&mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    fn get_uri(request_line: &String) -> String {
        request_line.split_whitespace().nth(1).unwrap().to_string()
    }

    fn get_http_method(request_line: &String) -> String {
        request_line.split_whitespace().nth(0).unwrap().to_string()
    }

    fn get_echo_path(uri: &String) -> String {
        uri.split("/").nth(2).unwrap_or("").to_string()
    }

    fn get_header_map(request_line: &String) -> HashMap<String, String> {
        let mut request_map: HashMap<String, String> = HashMap::new();
        let res: Vec<_> = request_line.split("\n")
                        .filter_map(|l|                         
                            Some(
                                l.split_once(":")
                                .map(|l| (l.0.to_string(), l.1.trim().to_string()))
                            )
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
        let uri = Self::get_uri(&request_line);
        let file_name = Self::get_file_name(&uri);
        let http_method = Self::get_http_method(&request_line);
        let body = Self::get_http_body(&mut buf_reader, &request_line);
        let headers_map = Self::get_header_map(&request_line);
        let encoding = headers_map.get("Accept-Encoding")
                                    .map(|e| e.split(",").map(|e| e.trim().to_string()).collect()).unwrap_or(vec![]);
        let user_agent = headers_map.get("User-Agent").map(|v| v.to_string());
        let echo_path = Self::get_echo_path(&uri);

        Self {
            uri,
            http_method,
            encoding,
            user_agent,
            echo_path,
            body,
            file_name,
        }
    }
}

struct Response {
    status: HttpCode,
    content: ContentType,
    encoding: Vec<String>,
    body: String,
}

fn get_valid_encoding(request_encodings: &Vec<String>) -> Option<String>  {
    let VALID = ["gzip"];
    let joined = request_encodings
            .iter()
            .map(String::as_str)
            .filter(
                |e| VALID.contains(e)
            )
            .collect::<Vec<_>>()
            .join(",");
    (!joined.is_empty()).then(|| joined)
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let encoding = get_valid_encoding(&self.encoding);
        let encoding_content = match encoding {
            Some(e) => format!("Content-Encoding: {}\r\n", e),
            None => "".to_string()
        };
        write!(f, "{status}\r\nContent-Type: {content}\r\n{encoding_content}Content-Length: {len}\r\n\r\n{body}",
            status=self.status,
            len=self.body.len(),
            content=self.content,
            body=self.body.clone()
        )
    }
}

fn main() {
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

fn create_file(file_name: &str, content: &String) -> Result<(), Error> {
    let file_path = format!("/tmp/data/codecrafters.io/http-server-tester/{file_name}");
    let file = fs::File::create(file_path);
    file.expect("File not found!").write_all(content.as_bytes())
}

fn handle_files_routes(req: Request) -> Response {
    match req.http_method.as_str() {
        "POST" => match create_file(&req.file_name, &req.body) {
            Ok(_) => Response {
                status: HttpCode::Created, 
                content: ContentType::None, 
                encoding: req.encoding,
                body:"".to_string()
            },
            Err(_) => Response {
                status: HttpCode::NotFound, 
                content: ContentType::None, 
                encoding: req.encoding,
                body:"".to_string()
            }
        },
        _ => match read_file(&req.file_name) {
            Ok(content) => Response {
                status: HttpCode::OK, 
                content: ContentType::Octet, 
                encoding: req.encoding,
                body: content
            },
            Err(_) => Response {
                status: HttpCode::NotFound, 
                content: ContentType::Text, 
                encoding: req.encoding,
                body: "".to_string()
            }
        }
    }
}

fn handle_routes(req: Request) -> Response {
    if req.uri == "/" {
        Response {
            status: HttpCode::OK, 
            content: ContentType::Text, 
            encoding: req.encoding,
            body: "".to_string()
        }
    } else if req.uri.starts_with("/echo") {
        Response {
            status: HttpCode::OK, 
            content: ContentType::Text, 
            encoding: req.encoding,
            body: req.echo_path.to_string()
        }
    } else if req.uri.starts_with("/user-agent") {
        Response {
            status: HttpCode::OK, 
            content: ContentType::Text, 
            encoding: req.encoding,
            body: req.user_agent.clone().unwrap()
        }
    } else if req.uri.starts_with("/files") {
        handle_files_routes(req)
    } else {
        Response {
            status: HttpCode::NotFound, 
            content: ContentType::Text, 
            encoding: req.encoding,
            body: "".to_string()
        }
    }
}

fn handle_client(stream: TcpStream){
    println!("accepted new connection");
    let mut stream = stream;
    let req: Request;
    {
        let stream2 = &mut stream;
        req = Request::new(stream2);
    }
    let resp = get_valid_encoding(&req.encoding);
    let response = handle_routes(req);
    
    stream.write_all(format!("{response}").as_bytes()).unwrap();
}
