use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read,}, 
    net::TcpStream,
};

use crate::utils::gzip_data;

#[derive(Debug)]
pub struct Request {
    pub uri: String,
    pub http_method: String,
    pub encoding: Option<String>,
    pub user_agent: Option<String>,
    pub echo_str: String,
    pub body: String,
    pub file_name: String,
    pub should_compress: bool,
}

impl Request {
    fn extract_http_body(buf_reader: &mut BufReader<&mut TcpStream>, request_line: &String) -> String {
        let size = request_line.lines()
                .find_map(|l| 
                    l.split_once(":")
                    .filter(|(k, _)| k.starts_with("Content-Length"))
                    .and_then(|(_, v)| v.trim().parse::<usize>().ok())
                ).unwrap_or(0);

        let mut buffer = vec![0; size]; 
        buf_reader.read_exact(&mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    fn get_uri(request_line: &String) -> String {
        request_line.split_whitespace().nth(1).unwrap_or("").to_string()
    }

    fn extract_http_method(request_line: &String) -> String {
        request_line.split_whitespace().nth(0).unwrap_or("").to_string()
    }

    fn get_header_map(request_line: &String) -> HashMap<String, String> {
        request_line.lines()
            .filter_map(|l|                         
                l.split_once(":")
                .map(|(k,v)| (k.to_string(), v.trim().to_string()))
            )
            .collect()
    }

    fn extract_file_name(uri: &String) -> String {
        uri.split("/").nth(2).unwrap_or("").to_string()
    }

    fn get_valid_encoding(headers_map: &HashMap<String, String>) -> (bool, Option<String>)  {
        let valid_encs = ["gzip"];
        let encodings: Vec<String> = headers_map.get("Accept-Encoding")
            .map(|e| 
                e.split(",")
                    .map(str::trim)
                    .map(String::from)
                    .collect()
            )
            .unwrap_or_default();

        let joined = encodings
            .iter()
            .map(String::as_str)
            .filter(
                |e| valid_encs.contains(e)
            )
            .collect::<Vec<_>>()
            .join(",");
        
        let should_compress = !encodings.is_empty();
        let encoding = (!joined.is_empty()).then(|| joined);
        (should_compress, encoding)
    }

    fn extract_user_agent(headers_map: &HashMap<String, String>) -> Option<String> {
        headers_map.get("User-Agent").map(|v| v.to_string())
    }

    pub fn get_compressed_body(&self) -> Option<Vec<u8>> {
        self.should_compress
            .then(|| gzip_data(self.echo_str.as_bytes()).unwrap())
            .or(None)
    }

    pub fn echo_str(&self) -> String {
        (!self.should_compress).then(|| self.echo_str.to_string()).unwrap_or("".to_string())
    }

    fn extract_echo_str(uri: &String) -> String {
        uri.split("/").nth(2).unwrap_or("").to_string()
    }

    pub fn new(stream: &mut TcpStream) -> Self {
        let mut buf_reader = BufReader::new(stream);
        
        let mut request_line = String::new();
        loop {
            let r = buf_reader.read_line(&mut request_line).unwrap();
            if r < 3 {
                break;
            }
        }
        let uri = Self::get_uri(&request_line);
        let headers_map = Self::get_header_map(&request_line);
        let (should_compress, encoding) = Self::get_valid_encoding(&headers_map);
        let user_agent = Self::extract_user_agent(&headers_map);

        let file_name = Self::extract_file_name(&uri);
        let echo_str = Self::extract_echo_str(&uri);
        let http_method = Self::extract_http_method(&request_line);
        let body = Self::extract_http_body(&mut buf_reader, &request_line);

        Self {
            uri,
            http_method,
            encoding,
            user_agent,
            echo_str,
            body,
            file_name,
            should_compress,
        }
    }
}
