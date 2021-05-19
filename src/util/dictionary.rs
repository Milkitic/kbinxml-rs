use std::io::Error;
use encoding::Encoding;

use crate::types::BoxResult;

pub struct EncodingDictionary {}

impl EncodingDictionary {
    pub fn get_encode_flag(encoding: &dyn Encoding) -> BoxResult<i32> {
        if encoding.whatwg_name().is_none() {
            return Err(Box::new(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unsupported encoding: {}", encoding.name()),
            )));
        }

        let name = encoding.whatwg_name().unwrap();
        match name {
            "ascii" => Ok(0x20),
            "windows-1252" => Ok(0x40),
            "euc-jp" => Ok(0x60),
            "shift_jis" => Ok(0x80),
            "utf-8" => Ok(0xA0),
            _ => Err(Box::new(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unsupported encoding: {}", name),
            ))),
        }
    }
}