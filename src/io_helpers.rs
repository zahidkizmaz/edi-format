use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
};

use tracing::{debug, info};

pub fn read_una_content(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut buffer = [0; 9];
    file.read_exact(&mut buffer).unwrap();
    let una = String::from_utf8_lossy(&buffer).into();
    debug!("found una: {}", una);
    una
}

pub fn read_content_from_file(file_path: &str) -> String {
    debug!("reading file: {}", file_path);
    let mut content = "".to_string();
    let mut file = File::open(file_path).unwrap();
    file.read_to_string(&mut content).unwrap();
    content.trim().to_string()
}

pub fn read_content_from_stdin() -> String {
    let mut buffer = String::new();
    let mut handle = stdin().lock();
    handle.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn write_content_to_file(file_path: &str, content: String) -> Result<(), ()> {
    info!("formatting file: {}", file_path);
    let mut file = File::create(file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    Ok(())
}

pub fn write_content_to_stdout(content: String) -> Result<(), ()> {
    let mut handle = stdout().lock();
    handle.write_all(content.as_bytes()).unwrap();
    Ok(())
}
