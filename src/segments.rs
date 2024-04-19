#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub struct UNA {
    composite_element_delimiter: char,
    data_element_delimiter: char,
    decimal_comma: char,
    escape_character: char,
    reserved_space: char,
    pub segment_delimiter: char,
}

impl Default for UNA {
    fn default() -> Self {
        Self {
            composite_element_delimiter: ':',
            data_element_delimiter: '+',
            decimal_comma: '.',
            escape_character: '?',
            reserved_space: ' ',
            segment_delimiter: '\'',
        }
    }
}

impl From<String> for UNA {
    fn from(value: String) -> Self {
        if value.len() != 9 {
            panic!(
                "UNA segment does not have 9 characters. {line}, {length}",
                length = value.len(),
                line = value
            );
        }
        UNA::parse(value.chars().collect::<Vec<char>>().try_into().unwrap())
    }
}

impl UNA {
    fn parse(line: [char; 9]) -> Self {
        Self {
            composite_element_delimiter: line[3],
            data_element_delimiter: line[4],
            decimal_comma: line[5],
            escape_character: line[6],
            reserved_space: line[7],
            segment_delimiter: line[8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_una() {
        let una = UNA::default();

        assert_eq!(una.composite_element_delimiter, ':');
        assert_eq!(una.data_element_delimiter, '+');
        assert_eq!(una.decimal_comma, '.');
        assert_eq!(una.escape_character, '?');
        assert_eq!(una.reserved_space, ' ');
        assert_eq!(una.segment_delimiter, '\'');
    }

    #[test]
    fn parse_valid_line_to_una() {
        let line = String::from("UNA:+.? '");

        let una: UNA = line.into();

        assert_eq!(una.composite_element_delimiter, ':');
        assert_eq!(una.data_element_delimiter, '+');
        assert_eq!(una.decimal_comma, '.');
        assert_eq!(una.escape_character, '?');
        assert_eq!(una.reserved_space, ' ');
        assert_eq!(una.segment_delimiter, '\'');
    }
}
