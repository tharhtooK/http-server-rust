use crate::file_handler::{create_file, get_args, get_directory, read_file};
use crate::request::Request;

use crate::response::{ContentType, HttpCode, Response};

fn _create_file(dir: String, req: &Request) -> Response {
    match create_file(dir, &req.file_name, &req.body) {
        Err(_) => Response::new(req),
        Ok(_) => {
            let body = req.body.to_string();
            Response::new(req)
                .status(HttpCode::Created)
                .content(ContentType::None)
                .body(body)
        }
    }
}

fn _read_file(dir: String, req: &Request) -> Response {
    match read_file(dir, &req.file_name) {
        Err(_) => Response::new(req),
        Ok(content) => {
            let encoding = req.encoding.to_owned();
            Response::new(req)
                .status(HttpCode::OK)
                .content(ContentType::Octet)
                .encoding(encoding)
                .body(content)
        }
    }
}

fn route_files(req: Request) -> Response {
    let dir = get_directory(&get_args());
    match req.http_method.as_str() {
        "POST" => _create_file(dir, &req),
        _ => _read_file(dir, &req),
    }
}

fn route_echo(req: &Request) -> Response {
    let encoding = req.encoding.clone();
    let echo_str = req.echo_str();
    let compressed = req.get_compressed_body();

    Response::new(req)
        .status(HttpCode::OK)
        .encoding(encoding)
        .body(echo_str)
        .compressed_body(compressed)
}

fn route_user_agent(req: &Request) -> Response {
    Response::new(req)
        .status(HttpCode::OK)
        .body(req.user_agent.clone().unwrap())
}

pub fn handle_routes(req: Request) -> Response {
    if req.uri == "/" {
        Response::new(&req).status(HttpCode::OK)
    } else if req.uri.starts_with("/echo") {
        route_echo(&req)
    } else if req.uri.starts_with("/files") {
        route_files(req)
    } else if req.uri.starts_with("/user-agent") {
        route_user_agent(&req)
    } else {
        Response::new(&req)
    }
}
