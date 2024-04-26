use crate::io_helpers::{self, read_content_from_file};
use crate::segments::UNA;

pub struct EDIFormatter {
    una: UNA,
    pub file_content: String,
}

impl EDIFormatter {
    pub fn new(file_path: &str) -> Self {
        let una = UNA::from(io_helpers::read_una_content(file_path));
        let file_content = read_content_from_file(file_path);
        Self { una, file_content }
    }

    fn format_segment(&self, segment: &str) -> Option<String> {
        if !segment.is_empty() {
            return Some(format!(
                "{s}{d}",
                s = segment,
                d = self.una.segment_delimiter
            ));
        }
        None
    }

    pub fn format(&self) -> String {
        self.file_content
            .split(self.una.segment_delimiter)
            .filter_map(|s| self.format_segment(s))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::io_helpers::read_content_from_file;

    use super::*;

    #[test]
    fn read_valid_una_from_file() {
        let file_path = "tests/valid_formatted.edi";
        let formatter = EDIFormatter::new(file_path);

        assert_eq!(formatter.una, UNA::from(String::from("UNA:+.? '")));
    }

    #[test]
    fn formatted_content() {
        let formatted_file_path = "tests/valid_formatted.edi";
        let unformatted_file_path = "tests/valid_not_formatted.edi";

        let formatter = EDIFormatter::new(unformatted_file_path);

        assert_eq!(
            formatter.format(),
            read_content_from_file(formatted_file_path)
        );
    }
}
