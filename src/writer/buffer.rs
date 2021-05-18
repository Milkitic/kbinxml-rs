use std::io::{Result, Write};

use crate::traits::BufferWrite;

pub struct NodeBufferWriter {
    stream: Vec<u8>,
}

impl NodeBufferWriter {
    pub fn new() -> Self {
        Self { stream: Vec::new() }
    }
}

impl BufferWrite for NodeBufferWriter {
    fn write_bytes(&mut self, buffer: &[u8]) -> Result<()> {
        let result = self.stream.write(buffer);
        if result.is_ok() {
            Ok(())
        } else {
            Err(result.unwrap_err())
        }
    }

    fn to_bytes(&self) -> &Vec<u8> {
        &self.stream
    }
}

pub struct DataBufferWriter {
    stream: Vec<u8>,
    pos8: i32,
    pos16: i32,
    pos32: i32,
}

impl DataBufferWriter {
    pub fn new() -> Self {
        Self {
            stream: Vec::new(),
            pos8: 0,
            pos16: 0,
            pos32: 0,
        }
    }

    fn pad(&mut self) -> Result<()> {
        let vec = &mut self.stream;
        while vec.len() % 4 != 0 {
            let result = vec.write(&[0]);
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        Ok(())
    }
}

impl BufferWrite for DataBufferWriter {
    fn write_bytes(&mut self, buffer: &[u8]) -> Result<()> {
        let result = self.stream.write(buffer);
        if result.is_ok() {
            Ok(())
        } else {
            Err(result.unwrap_err())
        }
    }

    fn to_bytes(&self) -> &Vec<u8> {
        &self.stream
    }
}
