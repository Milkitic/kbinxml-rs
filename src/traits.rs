pub trait MyWriter {
    fn write(self, s: &str) -> Vec<u8>;
}
