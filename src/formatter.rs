use std::io::{self, BufRead, BufReader, Read, Write};

use crate::segments::UNA;

pub(crate) fn format(mut input: impl Read, mut output: impl Write) -> Result<(), io::Error> {
    let una = UNA::from(&mut input)?;
    let mut take_over_from_last_line: Option<u8> = None;

    let mut line_break = [0u8; 1];
    loop {
        let read = input.read(&mut line_break)?;
        if read == 0 {
            break;
        } else if line_break == [b'\n'] {
            continue;
        } else {
            take_over_from_last_line = Some(line_break[0]);
            break;
        }
    }

    una.write_to(&mut output)?;
    let written = output.write(b"\n")?;
    debug_assert_eq!(written, 1);

    let mut input = BufReader::new(input);

    let mut buf = Vec::new();
    loop {
        buf.clear();
        if let Some(toflm) = take_over_from_last_line {
            buf.push(toflm);
            take_over_from_last_line = None;
        }
        input.read_until(una.segment_delimiter, &mut buf)?;

        let mut buf_len = buf.len();
        while buf_len > 0 && buf[buf_len - 1] == b'\n' {
            buf_len -= 1;
        }

        let mut line_break = [0u8; 1];
        loop {
            let read = input.read(&mut line_break)?;
            if read == 0 {
                break;
            } else if line_break == [b'\n'] {
                continue;
            } else {
                take_over_from_last_line = Some(line_break[0]);
                break;
            }
        }

        if buf.is_empty() || buf_len == 0 {
            break;
        }

        output.write_all(&buf[..buf_len])?;
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
