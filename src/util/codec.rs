use std::io::Error;

use crate::types::BoxResult;
pub struct Sixbit {}

impl Sixbit {
    // const CHARSET: Set<char> = phf_set! {
    //     '0', '1', '2', '3', '4', '5', '6', '7',
    //     '8', '9', ':', 'A', 'B', 'C', 'D', 'E',
    //     'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    //     'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    //     'V', 'W', 'X', 'Y', 'Z', '_', 'a', 'b',
    //     'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    //     'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    //     's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    // };

    pub fn encode(input: &str) -> BoxResult<Vec<u8>> {
        let bytes = input.as_bytes();
        let mut buffer: Vec<u8> = vec![0; bytes.len()];
        for i in 0..bytes.len() {
            buffer[i] = Sixbit::demap(&bytes[i])?;
        }
        let length = (buffer.len() as f64 * 6.0 / 8.0).ceil() as i32;
        let mut output: Vec<u8> = vec![0; length as usize];

        for i in 0..(buffer.len() * 6) {
            output[i / 8] =
                (output[i / 8] | ((buffer[i / 6] >> (5 - (i % 6)) & 1) << (7 - (i % 8)))) as u8;
        }

        Ok(output)
    }

    fn demap(source_char: &u8) -> BoxResult<u8> {
        if source_char >= &b'a' && source_char <= &b'z' {
            return Ok(source_char - 86u8);
        } else if source_char >= &b'A' && source_char <= &b'Z' {
            return Ok(source_char - 54u8);
        } else if source_char >= &b'0' && source_char <= &b'9' {
            return Ok(source_char - 48u8);
        } else if source_char == &b':' {
            return Ok(10u8);
        } else if source_char == &b'_' {
            return Ok(37u8);
        }

        return Err(Box::new(Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Sixbit unsupported char: `{}`", source_char.to_string()),
        )));
    }
}
