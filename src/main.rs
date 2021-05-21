mod tests;

use crate::traits::BinWriter;
use writer::KBinWriter;

mod cextern;
mod kbin_encodings;
mod traits;
mod types;
mod util;
mod writer;

use std::{fs::File, io::Read};

fn main() {
    let result = KBinWriter::new_with_code_name("shift_jis");
    if result.is_ok() {
        let mut file =
            File::open("G:\\GitHub\\kbinxmlcs\\kbinxmlcs.Test\\bin\\Debug\\net5.0\\huge.xml")
                .unwrap();
        // let file = BufReader::new(file);
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        if buffer.starts_with("\u{feff}") {
            buffer.remove(0);
        }

        let writer: KBinWriter = result.unwrap();
        let array = writer.write(&buffer).unwrap();

        let g = array.len();
        println!("{}", g);
    } else {
        println!("NOOOOO!!!");
    }
}
