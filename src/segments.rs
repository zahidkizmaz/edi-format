use std::io::Read;
use std::io::{self, Write};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub struct UNA {
    composite_element_delimiter: u8,
    data_element_delimiter: u8,
    decimal_comma: u8,
    escape_character: u8,
    reserved_space: u8,
    pub segment_delimiter: u8,
}

impl UNA {
    pub(crate) fn from(mut input: impl Read) -> Result<Self, io::Error> {
        let mut line = [0; 9];
        input.read_exact(&mut line)?;
        Ok(Self {
            composite_element_delimiter: line[3],
            data_element_delimiter: line[4],
            decimal_comma: line[5],
            escape_character: line[6],
            reserved_space: line[7],
            segment_delimiter: line[8],
        })
    }

    pub(crate) fn write_to(&self, mut output: impl Write) -> Result<(), io::Error> {
        output.write_all(b"UNA")?;
        output.write_all(&[
            self.composite_element_delimiter,
            self.data_element_delimiter,
            self.decimal_comma,
            self.escape_character,
            self.reserved_space,
            self.segment_delimiter,
        ])?;
        Ok(())
    }
}

impl Default for UNA {
    fn default() -> Self {
        Self {
            composite_element_delimiter: b':',
            data_element_delimiter: b'+',
            decimal_comma: b'.',
            escape_character: b'?',
            reserved_space: b' ',
            segment_delimiter: b'\'',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_una() {
        let una = UNA::default();

        assert_eq!(una.composite_element_delimiter, b':');
        assert_eq!(una.data_element_delimiter, b'+');
        assert_eq!(una.decimal_comma, b'.');
        assert_eq!(una.escape_character, b'?');
        assert_eq!(una.reserved_space, b' ');
        assert_eq!(una.segment_delimiter, b'\'');
    }

    #[test]
    fn parse_valid_line_to_una() {
        let line = b"UNA:+.? '";

        let una = UNA::from(&line[..]).unwrap();

        assert_eq!(una.composite_element_delimiter, b':');
        assert_eq!(una.data_element_delimiter, b'+');
        assert_eq!(una.decimal_comma, b'.');
        assert_eq!(una.escape_character, b'?');
        assert_eq!(una.reserved_space, b' ');
        assert_eq!(una.segment_delimiter, b'\'');
    }
}
