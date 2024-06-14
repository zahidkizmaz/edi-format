use std::io::{self, BufRead, BufReader, Read, Write};

use tracing::debug;

use crate::segments::UNA;

fn skip_over_line_breaks(input: &mut impl BufRead) -> Result<(), io::Error> {
    loop {
        let rest = input.fill_buf()?;
        if rest.is_empty() {
            break;
        }
        if rest[0] == b'\n' {
            input.consume(1);
        } else {
            break;
        }
    }
    Ok(())
}

pub(crate) fn format(input: impl Read, mut output: impl Write) -> Result<(), io::Error> {
    let mut input = BufReader::new(input);

    let una = UNA::from(&mut input)?;
    skip_over_line_breaks(&mut input)?;

    una.write_to(&mut output)?;
    let written = output.write(b"\n")?;
    debug_assert_eq!(written, 1);

    let mut buf = Vec::new();
    loop {
        buf.clear();
        input.read_until(una.segment_delimiter, &mut buf)?;
        debug!(
            segment = %String::from_utf8_lossy(&buf),
            "Formatting segment"
        );
        skip_over_line_breaks(&mut input)?;

        if buf.is_empty() {
            break;
        }

        output.write_all(&buf)?;
        let written = output.write(b"\n")?;
        debug_assert_eq!(written, 1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions_sorted::assert_eq_sorted;

    #[test]
    fn formatted_content() {
        let test_input = include_bytes!("../tests/valid_not_formatted.edi");
        let test_output = include_bytes!("../tests/valid_formatted.edi");

        let mut buf = Vec::new();
        format(&mut io::Cursor::new(&test_input), &mut buf).unwrap();

        assert_eq_sorted!(
            test_output,
            &buf[..],
            r#"expected: "{}", actual: "{}""#,
            String::from_utf8_lossy(test_output),
            String::from_utf8_lossy(&buf),
        );
    }

    #[test]
    fn formatted_content_twice() {
        let test_input = include_bytes!("../tests/valid_formatted.edi");

        let mut buf = Vec::new();
        format(&mut io::Cursor::new(&test_input), &mut buf).unwrap();

        assert_eq_sorted!(
            test_input,
            &buf[..],
            r#"expected: "{}", actual: "{}""#,
            String::from_utf8_lossy(test_input),
            String::from_utf8_lossy(&buf),
        );
    }
}
