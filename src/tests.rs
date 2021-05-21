#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use std::{fs::File, io::Read};

    use encoding::Encoding;
    use quick_xml::{events::Event, Reader};

    use crate::{
        traits::{BigEndianBinaryWrite, BinWriter},
        types::{BoxResult, StringToByteFunc},
        util::converter::ValueConverter,
        writer::{buffer::DataBufferWriter, KBinWriter},
    };

    fn indent(size: usize) -> String {
        const INDENT: &'static str = "    ";
        (0..size)
            .map(|_| INDENT)
            .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
    }

    fn get_string(raw: &[u8], encoding: &dyn Encoding) -> BoxResult<String> {
        Ok(encoding.decode(raw, encoding::DecoderTrap::Replace)?)
    }

    #[test]
    fn read_xml() {
        let file = File::open("anotest.xml").unwrap();
        let file = BufReader::new(file);
        let mut encoding: &(dyn Encoding + Send + Sync) =
            encoding::label::encoding_from_whatwg_label("utf-8").unwrap();
        let mut reader = Reader::from_reader(file);
        reader.trim_text(true);
        let mut depth = 0;
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = get_string(&e.local_name(), encoding).unwrap();
                    println!("{}+`{}`", indent(depth), name);
                    depth += 1;

                    for attr in e.attributes() {
                        let result = attr.unwrap();
                        let attr_name = get_string(result.key, encoding).unwrap();
                        let attr_val = get_string(&result.value, encoding).unwrap();
                        println!("{}[{}]=`{}`", indent(depth), attr_name, attr_val);
                    }
                }
                Ok(Event::End(e)) => {
                    let name = get_string(e.local_name(), encoding).unwrap();
                    depth -= 1;
                    println!("{}-{}", indent(depth), name);
                }
                Ok(Event::Decl(e)) => {
                    let enc = e.encoding().unwrap().unwrap();
                    let enc_str = get_string(&enc, encoding).unwrap();
                    encoding = encoding::label::encoding_from_whatwg_label(&enc_str).unwrap();
                    // if let Some(b) = e.standalone() {
                    //     println!("has value {}", b.unwrap());
                    // }

                    println!("doc declaration");
                }
                Ok(Event::Text(e)) => {
                    let u8 = e.unescaped().unwrap();
                    let value = get_string(&u8, encoding).unwrap();
                    println!("{}VALUE: `{}`", indent(depth), value);
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }

    #[test]
    fn mine() {
        let writer = KBinWriter::new_with_code_name("shift_jis").unwrap();
        writer.write("<test></test>");

        let val = ValueConverter::u32_to_bytes("1242342134343212341");
        if val.is_ok() {
            println!("{:?}", val.unwrap());
        } else {
            let e = val.unwrap_err();
            eprintln!("error: {}", e);
        }
    }

    #[test]
    fn write_buffers() {
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

            let writer = result.unwrap();
            let array = writer.write(&buffer).unwrap();

            let g = array.len();
        } else {
            println!("NOOOOO!!!");
        }
        // let dbw = DataBufferWriter::new();
        // let bytes = dbw.to_bytes();
        // let b = bytes.to_owned();
        // println!("{:?}", b);
    }
}
