use std::error::Error;

pub type StringToByteFunc = fn(&str) -> BoxResult<Vec<u8>>;
pub type ByteToStringFunc = fn([u8]) -> String;

pub type BoxResult<T> = Result<T, Box<dyn Error>>;
