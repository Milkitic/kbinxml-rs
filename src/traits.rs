use std::io::Result;

pub trait BinWriter {
    fn write(self, s: &str) -> Vec<u8>;
}

pub trait BufferWrite {
    fn to_bytes(&self) -> &Vec<u8>;

    fn write_bytes(&mut self, buffer: &[u8]) -> Result<()>;

    fn write_s8(&mut self, value: i8) -> Result<()> {
        Self::write_bytes(self, &[value as u8])
    }

    fn write_s16(&mut self, value: i16) -> Result<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }

    fn write_s32(&mut self, value: i32) -> Result<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }

    fn write_s64(&mut self, value: i64) -> Result<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }

    fn write_u8(&mut self, value: u8) -> Result<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }

    fn write_u16(&mut self, value: u16) -> Result<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }

    fn write_u32(&mut self, value: u32) -> Result<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }

    fn write_u64(&mut self, value: u64) -> Result<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }
}
