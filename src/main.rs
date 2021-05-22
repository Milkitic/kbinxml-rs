mod tests;

use crate::traits::BinWriter;
use crate::util::codec::Sixbit;
use stopwatch::Stopwatch;
use writer::KBinWriter;

mod cextern;
mod kbin_encodings;
mod traits;
mod types;
mod util;
mod writer;

use std::{fs::File, io::{Read, Write}};

fn main() {
    let sw = Stopwatch::start_new();
    let count = 20000000;
    let mut vec: Vec<u8> = Vec::new();
    for i in 0..count {
        let mut vec2: Vec<u8> = vec![0, 242, 234, 42, 132, 223, 5];
        vec.append(&mut vec2);
    }
    println!("{},{}",vec.len(), sw.elapsed_ms());
    return;
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
