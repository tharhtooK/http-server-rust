use crate::request::Request;
use crate::file_handler::{
    create_file, 
    read_file,
};

use crate::response::{
    HttpCode, 
    ContentType, 
    Response,
};

fn _create_file(req: &Request) -> Response {
    match create_file(&req.file_name, &req.body) {
        Err(_) => Response::new(),
        Ok(_) => {
            let body = req.body.to_string();
            Response::new()
                .status(HttpCode::Created)
                .content(ContentType::None)
                .body(body)
        }
    }
}

fn _read_file(req: &Request) -> Response {
    match read_file(&req.file_name) {
        Err(_) => Response::new(),
        Ok(content) => {
            let encoding = req.encoding.to_owned();
            Response::new()
                .status(HttpCode::OK)
                .content(ContentType::Octet)
                .encoding(encoding)
                .body(content)
        }
    }
}

fn route_files(req: Request) -> Response {
    match req.http_method.as_str() {
        "POST" => _create_file(&req),
        _ => _read_file(&req)
    }
}

fn route_echo(req: &Request) -> Response {
    let encoding = req.encoding.clone();
    let echo_str = req.echo_str();
    let compressed = req.get_compressed_body();

    Response::new()
        .status(HttpCode::OK)
        .encoding(encoding)
        .body(echo_str)
        .compressed_body(compressed)
}

fn route_user_agent(req: &Request) -> Response {
    Response::new()
        .status(HttpCode::OK)
        .body(req.user_agent.clone().unwrap())
}

pub fn handle_routes(req: Request) -> Response {
    if req.uri == "/" {
        Response::new().status(HttpCode::OK)
    } else if req.uri.starts_with("/echo") {
        route_echo(&req)
    } else if req.uri.starts_with("/files") {
        route_files(req)
    } else if req.uri.starts_with("/user-agent") {
        route_user_agent(&req)
    } else {
        Response::new()
    }
}
