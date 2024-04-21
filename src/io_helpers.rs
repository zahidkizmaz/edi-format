use std::{
    fs::File,
    io::{Read, Write},
};

pub fn read_una_content(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut buffer = [0; 9];
    file.read_exact(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer).into()
}

pub fn read_content_from_file(file_path: &str) -> String {
    let mut content = "".to_string();
    let mut file = File::open(file_path).unwrap();
    file.read_to_string(&mut content).unwrap();
    content.trim().to_string()
}

pub fn write_content_to_file(file_path: &str, content: String) -> Result<usize, ()> {
    let mut file = File::open(file_path).unwrap();
    Ok(file.write(content.as_bytes()).unwrap())
}