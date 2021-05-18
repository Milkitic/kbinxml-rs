use std::{io::Error, net::Ipv4Addr};

use crate::types::BoxResult;

pub struct ValueConverter {}

impl ValueConverter {
    pub fn u8_to_bytes(str: String) -> BoxResult<[u8; 1]> {
        let result = str.parse::<u8>()?;
        Ok([result])
    }

    pub fn s8_to_bytes(str: String) -> BoxResult<[u8; 1]> {
        let result = str.parse::<i8>()?;
        Ok([result as u8])
    }

    pub fn u16_to_bytes(str: String) -> BoxResult<[u8; 2]> {
        let result = str.parse::<u16>()?;
        let bytes = result.to_be_bytes();
        Ok(bytes)
    }

    pub fn s16_to_bytes(str: String) -> BoxResult<[u8; 2]> {
        let result = str.parse::<i16>()?;
        let bytes = result.to_be_bytes();
        Ok(bytes)
    }

    pub fn u32_to_bytes(str: String) -> BoxResult<[u8; 4]> {
        let result = str.parse::<u32>()?;
        let bytes = result.to_be_bytes();
        Ok(bytes)
    }

    pub fn s32_to_bytes(str: String) -> BoxResult<[u8; 4]> {
        let result = str.parse::<i32>()?;
        let bytes = result.to_be_bytes();
        Ok(bytes)
    }

    pub fn u64_to_bytes(str: String) -> BoxResult<[u8; 8]> {
        let result = str.parse::<u64>()?;
        let bytes = result.to_be_bytes();
        Ok(bytes)
    }

    pub fn s64_to_bytes(str: String) -> BoxResult<[u8; 8]> {
        let result = str.parse::<i64>()?;
        let bytes = result.to_be_bytes();
        Ok(bytes)
    }

    pub fn single_to_bytes(str: String) -> BoxResult<[u8; 4]> {
        let result = str.parse::<f32>()?;
        let bytes = result.to_be_bytes();
        Ok(bytes)
    }

    pub fn double_to_bytes(str: String) -> BoxResult<[u8; 8]> {
        let result = str.parse::<f64>()?;
        let bytes = result.to_be_bytes();
        Ok(bytes)
    }

    pub fn ip4_to_bytes(str: String) -> BoxResult<[u8; 4]> {
        let result = str.parse::<Ipv4Addr>()?;
        let bytes = result.octets();
        Ok(bytes)
    }

    pub fn default(_str: String) -> BoxResult<[u8; 0]> {
        Err(Box::new(Error::new(
            std::io::ErrorKind::InvalidData,
            "You are using a default converter.",
        )))
    }
}
