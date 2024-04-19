use std::{fs::File, io::Read};

use crate::segments::UNA;

pub struct EDIFormatter {
    una: UNA,
    file_path: String,
}

impl EDIFormatter {
    fn new(file_path: String) -> Self {
        let una = UNA::from(EDIFormatter::read_una(&file_path));
        Self {
            una: una,
            file_path: file_path,
        }
    }

    fn read_una(file_path: &str) -> String {
        let mut file = File::open(file_path).unwrap();
        let mut buffer = [0; 9];
        file.read_exact(&mut buffer).unwrap();
        String::from_utf8_lossy(&buffer).into()
    }

    fn read_file(&self) -> String {
        let mut content = "".to_string();
        let mut file = File::open(&self.file_path).unwrap();
        file.read_to_string(&mut content).unwrap();
        content.trim().to_string()
    }

    fn format_content(&self) -> String {
        self.read_file()
            .split(self.una.segment_delimiter)
            .collect::<Vec<_>>()
            .join(format!("{d}\n", d = self.una.segment_delimiter).as_str())
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_valid_una_from_file() {
        let file_path = String::from("tests/valid_formatted.edi");

        let formatter = EDIFormatter::new(file_path);

        assert_eq!(formatter.una, UNA::from(String::from("UNA:+.? '")));
    }

    #[test]
    fn formatted_content() {
        let formatted_file_path = String::from("tests/valid_formatted.edi");
        let unformatted_file_path = String::from("tests/valid_not_formatted.edi");
        let formatter = EDIFormatter::new(unformatted_file_path);

        assert_eq!(
            formatter.format_content(),
            EDIFormatter::new(formatted_file_path).read_file()
        );
    }
}
