use crate::types::BoxResult;

pub trait BinWriter {
    fn write(self, s: &str) -> BoxResult<Vec<u8>>;
}

pub trait BigEndianBinaryWrite {
    fn to_bytes(&self) -> &Vec<u8>;

    fn write_bytes(&mut self, buffer: &[u8]) -> BoxResult<()>;

    fn write_s8(&mut self, value: i8) -> BoxResult<()> {
        Self::write_bytes(self, &[value as u8])
    }

    fn write_s16(&mut self, value: i16) -> BoxResult<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }

    fn write_s32(&mut self, value: i32) -> BoxResult<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }

    fn write_s64(&mut self, value: i64) -> BoxResult<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }

    fn write_u8(&mut self, value: u8) -> BoxResult<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }

    fn write_u16(&mut self, value: u16) -> BoxResult<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }

    fn write_u32(&mut self, value: u32) -> BoxResult<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }

    fn write_u64(&mut self, value: u64) -> BoxResult<()> {
        Self::write_bytes(self, &value.to_be_bytes())
    }
}
