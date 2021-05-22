use std::io::{Error, Result, Write};

use encoding::Encoding;

use crate::{traits::BigEndianBinaryWrite, types::BoxResult, util::codec::Sixbit};

pub struct NodeBufferWriter<'a> {
    stream: Vec<u8>,
    compressed: bool,
    whatwg_name: String,
    encoding: &'a dyn Encoding,
}

impl NodeBufferWriter<'_> {
    pub fn new_with_code_name(compressed: bool, code_name: &str) -> Result<Self> {
        let encoding = encoding::label::encoding_from_whatwg_label(code_name);
        if encoding.is_some() {
            let enc = encoding.unwrap();
            return Ok(Self {
                encoding: enc,
                whatwg_name: enc.whatwg_name().unwrap().to_string(),
                stream: Vec::new(),
                compressed,
            });
        } else {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find target encoding.",
            ));
        }
    }

    pub fn new_with_code_page(compressed: bool, code_page: usize) -> Result<Self> {
        let encoding = encoding::label::encoding_from_windows_code_page(code_page);
        if encoding.is_some() {
            let enc = encoding.unwrap();
            return Ok(Self {
                encoding: enc,
                whatwg_name: enc.whatwg_name().unwrap().to_string(),
                stream: Vec::new(),
                compressed,
            });
        } else {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find target encoding.",
            ));
        }
    }

    pub fn clear(&mut self) {
        self.stream.clear();
    }

    pub fn write_string(&mut self, s: &str) -> BoxResult<()> {
        if self.compressed {
            self.write_u8(s.len() as u8)?;
            self.write_bytes(&Sixbit::encode(s)?)?;
        } else {
            self.write_u8(((s.len() - 1) | (1 << 6)) as u8)?;
            self.write_bytes(&self.encoding.encode(s, encoding::EncoderTrap::Replace)?)?;
        }

        Ok(())
    }

    pub fn pad(&mut self) -> BoxResult<()> {
        let stream = &mut self.stream;
        let left = stream.len() % 4;
        if left != 0 {
            stream.append(&mut vec![0; 4 - left]);
        }

        Ok(())
    }

    /// Get a reference to the node buffer writer's whatwg name.
    pub fn whatwg_name(&self) -> &String {
        &self.whatwg_name
    }
}

impl BigEndianBinaryWrite for NodeBufferWriter<'_> {
    fn write_bytes(&mut self, buffer: &[u8]) -> BoxResult<()> {
        let _result = self.stream.write(buffer)?;
        Ok(())
    }

    fn to_bytes(&self) -> &Vec<u8> {
        &self.stream
    }
}

pub struct BigEndianBinaryWriter {
    stream: Vec<u8>,
}

impl BigEndianBinaryWriter {
    pub fn new() -> Self {
        Self { stream: Vec::new() }
    }
}

impl BigEndianBinaryWrite for BigEndianBinaryWriter {
    fn write_bytes(&mut self, buffer: &[u8]) -> BoxResult<()> {
        let _result = self.stream.write(buffer)?;
        Ok(())
    }

    fn to_bytes(&self) -> &Vec<u8> {
        &self.stream
    }
}

pub struct DataBufferWriter<'a> {
    stream: Vec<u8>,
    pos8: i32,
    pos16: i32,
    pos32: i32,
    encoding: &'a dyn Encoding,
}

impl DataBufferWriter<'_> {
    pub fn new_with_code_name(code_name: &str) -> Result<Self> {
        let encoding = encoding::label::encoding_from_whatwg_label(code_name);
        if encoding.is_some() {
            return Ok(Self {
                encoding: encoding.unwrap(),
                stream: Vec::new(),
                pos8: 0,
                pos16: 0,
                pos32: 0,
            });
        } else {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find target encoding.",
            ));
        }
    }

    pub fn new_with_code_page(code_page: usize) -> Result<Self> {
        let encoding = encoding::label::encoding_from_windows_code_page(code_page);
        if encoding.is_some() {
            return Ok(Self {
                encoding: encoding.unwrap(),
                stream: Vec::new(),
                pos8: 0,
                pos16: 0,
                pos32: 0,
            });
        } else {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find target encoding.",
            ));
        }
    }

    pub fn clear(&mut self) {
        self.stream.clear();
    }

    /// Get a reference to the data buffer writer's encoding.
    pub fn encoding(&self) -> &dyn Encoding {
        self.encoding
    }

    pub fn pad(&mut self) -> Result<()> {
        let stream = &mut self.stream;
        while stream.len() % 4 != 0 {
            let result = stream.write(&[0]);
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        Ok(())
    }

    pub fn write_string(&mut self, s: &str) -> BoxResult<()> {
        let mut vec = self.encoding.encode(s, encoding::EncoderTrap::Replace)?;
        vec.push(0);
        self.write_u32(vec.len() as u32)?;
        self.write_32bit_aligned(&mut vec)?;
        Ok(())
    }

    pub fn write_binary(&mut self, s: &str) -> BoxResult<()> {
        self.write_u32(s.len() as u32)?;
        let mut decoded = hex::decode(s)?;
        self.write_32bit_aligned(&mut decoded)?;
        Ok(())
    }

    fn write_32bit_aligned(&mut self, buffer: &mut Vec<u8>) -> BoxResult<()> {
        let stream = &mut self.stream;

        let len = self.pos32 - stream.len() as i32;
        if len > 0 {
            stream.append(&mut vec![0; len as usize])
        }

        self.pos32 = self.set_range(buffer, self.pos32);

        let left = self.pos32 % 4;
        if left != 0 {
            self.pos32 += 4 - left;
        }

        Ok(())
    }

    fn write_16bit_aligned(&mut self, buffer: &[u8]) -> BoxResult<()> {
        let stream = &mut self.stream;
        let len = self.pos16 - stream.len() as i32;
        if len > 0 {
            stream.append(&mut vec![0; len as usize])
        }

        if self.pos16 % 4 == 0 {
            self.pos32 += 4;
        }

        self.pos16 = self.set_range(&mut Vec::from(buffer), self.pos16);
        self.realign16_8();

        Ok(())
    }

    fn write_8bit_aligned(&mut self, buffer: &[u8]) -> BoxResult<()> {
        let stream = &mut self.stream;
        let len = self.pos8 - stream.len() as i32;
        if len > 0 {
            stream.append(&mut vec![0; len as usize])
        }

        if self.pos8 % 4 == 0 {
            self.pos32 += 4;
        }

        self.pos8 = self.set_range(&mut Vec::from(buffer), self.pos8);
        self.realign16_8();

        Ok(())
    }

    fn set_range(&mut self, buffer: &mut Vec<u8>, mut offset: i32) -> i32 {
        if offset == self.stream.len() as i32 {
            let len = buffer.len();
            self.stream.append(buffer);
            offset += len as i32;
        } else {
            for i in 0..buffer.len() {
                self.stream[offset as usize] = buffer[i];
                offset += 1;
            }
        }

        offset
    }

    fn realign16_8(&mut self) {
        if self.pos8 % 4 == 0 {
            self.pos8 = self.pos32;
        }

        if self.pos16 % 4 == 0 {
            self.pos16 = self.pos32;
        }
    }
}

impl BigEndianBinaryWrite for DataBufferWriter<'_> {
    fn write_bytes(&mut self, buffer: &[u8]) -> BoxResult<()> {
        match buffer.len() {
            1 => self.write_8bit_aligned(buffer),
            2 => self.write_16bit_aligned(buffer),
            _ => self.write_32bit_aligned(&mut Vec::from(buffer)),
        }
    }

    fn to_bytes(&self) -> &Vec<u8> {
        &self.stream
    }
}
