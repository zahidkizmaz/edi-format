use tracing::trace;

use crate::io_helpers::{self, read_content_from_file};
use crate::segments::UNA;

#[derive(Debug, PartialEq)]
pub enum FormatResult {
    Format(String),
    Skip,
}

pub struct EDIFormatter {
    una: UNA,
    file_content: String,
}

impl EDIFormatter {
    pub fn new(file_path: &str) -> Self {
        let una = UNA::from(io_helpers::read_una_content(file_path));
        let file_content = read_content_from_file(file_path);
        Self { una, file_content }
    }

    fn format_segment(&self, segment: &str) -> Option<String> {
        if !segment.is_empty() {
            let segment = format!("{s}{d}", s = segment, d = self.una.segment_delimiter)
                .trim()
                .to_string();
            trace!("Segment: {segment}");
            return Some(segment);
        }
        None
    }

    fn format_content(&self) -> String {
        self.file_content
            .split(self.una.segment_delimiter)
            .filter_map(|s| self.format_segment(s))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn format(&self) -> Result<FormatResult, ()> {
        let formatted_content = self.format_content();

        if self.file_content == formatted_content {
            return Ok(FormatResult::Skip);
        }
        Ok(FormatResult::Format(formatted_content))
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
            formatter.format_content(),
            read_content_from_file(formatted_file_path)
        );
    }

    #[test]
    fn formatted_content_twice() {
        let formatted_file_path = "tests/valid_formatted.edi";
        let formatted_content = read_content_from_file(formatted_file_path);

        let formatter = EDIFormatter::new(formatted_file_path);

        assert_eq!(formatter.format_content(), formatted_content);
        assert_eq!(formatter.format_content(), formatted_content);
    }

    #[test]
    fn format_not_formatted_file() {
        let not_formatted_file_path = "tests/valid_not_formatted.edi";
        let formatted_file_path = "tests/valid_formatted.edi";
        let formatted_content = read_content_from_file(formatted_file_path);

        let formatter = EDIFormatter::new(not_formatted_file_path);

        assert_eq!(
            formatter.format(),
            Ok(FormatResult::Format(formatted_content))
        );
    }

    #[test]
    fn format_already_formatted_file() {
        let formatted_file_path = "tests/valid_formatted.edi";

        let formatter = EDIFormatter::new(formatted_file_path);

        assert_eq!(formatter.format(), Ok(FormatResult::Skip));
    }
}
