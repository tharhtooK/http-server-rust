use std::{
    env, fs,
    io::{Error, Write},
};

pub fn get_args() -> Vec<String> {
    env::args().collect()
}

pub fn get_directory(args: &[String]) -> String {
    let mut dir = String::from(".");

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--directory" | "-d" => {
                if let Some(val) = args.next() {
                    dir = val;
                }
            }
            _ => println!("failed to get directory"),
        }
    }
    dir
}

pub fn read_file(directory: String, file_name: &str) -> Result<String, Error> {
    let file_path = format!("{directory}{file_name}");
    fs::read_to_string(file_path)
}

pub fn create_file(directory: String, file_name: &str, content: &String) -> Result<(), Error> {
    let file_path = format!("{directory}{file_name}");
    let file = fs::File::create(file_path);
    file.expect("File not found!").write_all(content.as_bytes())
}
