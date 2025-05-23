use std::fmt;

#[derive(Debug)]
pub enum HttpCode {
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

#[derive(Debug)]
pub enum ContentType {
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
pub struct Response {
    pub status: HttpCode,
    pub content: ContentType,
    pub encoding: Option<String>,
    pub content_length: usize,
    pub body: String,
    pub compressed_body: Option<Vec<u8>>,
}

impl Response {
    pub fn new() -> Response {
        Response { 
            status: HttpCode::NotFound,
            content: ContentType::Text,
            encoding: Some("".to_string()),
            content_length: 0,
            body: "".to_string(),
            compressed_body: None
        }
    }

    pub fn status(mut self, code: HttpCode) -> Self {
        self.status = code;
        self
    }

    pub fn content(mut self, content: ContentType) -> Self {
        self.content = content;
        self
    }

    pub fn encoding(mut self, encoding: Option<String>) -> Self {
        self.encoding = encoding;
        self
    }

    pub fn body(mut self, body: String) -> Self {
        self.body = body;
        self.content_length = Self::get_content_length(&self);
        
        self
    }

    pub fn compressed_body(mut self, compressed: Option<Vec<u8>>) -> Self {
        self.compressed_body = compressed.to_owned();
        self.content_length = Self::get_content_length(&self);

        self
    }

    fn get_content_length(&self) -> usize {
        let body_len = self.body.chars().count();
        let compressed_len = self.compressed_body.iter().flat_map(|s| s).count();

        let should_compressed = compressed_len.gt(&0);
        should_compressed.then(|| compressed_len).or(Some(body_len)).unwrap()
    }

    fn get_encoding_str(&self) -> String {
        match self.encoding {
            Some(ref e) => format!("Content-Encoding: {}\r\n", e),
            None => "".to_string()
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let encoding_content = self.get_encoding_str();

        write!(f, "{status}\r\nContent-Type: {content}\r\n{encoding_content}Content-Length: {len}\r\n\r\n{body}",
            status=self.status,
            len=self.content_length,
            content=self.content,
            body=self.body,
        )
    }
}