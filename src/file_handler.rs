use std::{
    fs,
    io::{Error, Write,},
};

pub fn read_file(file_name: &str) -> Result<String, Error> {
    let file_path = format!("/tmp/data/codecrafters.io/http-server-tester/{file_name}");
    fs::read_to_string(file_path)
}

pub fn create_file(file_name: &str, content: &String) -> Result<(), Error> {
    let file_path = format!("/tmp/data/codecrafters.io/http-server-tester/{file_name}");
    let file = fs::File::create(file_path);
    file.expect("File not found!").write_all(content.as_bytes())
}
