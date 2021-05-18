extern crate xml;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use xml::reader::{EventReader, XmlEvent};

    use crate::{
        datamapping::converter::ValueConverter,
        traits::{BinWriter, BigEndianBinaryWrite},
        types::StringToByteFunc,
        writer::{buffer::DataBufferWriter, KBinWriter},
    };

    fn indent(size: usize) -> String {
        const INDENT: &'static str = "    ";
        (0..size)
            .map(|_| INDENT)
            .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
    }

    #[test]
    fn read_xml() {
        let file = File::open("file.xml").unwrap();
        let file = BufReader::new(file);
        let mut conf = xml::reader::ParserConfig::new();
        conf.ignore_comments = true;
        let parser = EventReader::new_with_config(file, conf);
        let mut depth = 0;

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    println!("{}+{}", indent(depth), name);
                    depth += 1;

                    for attr in attributes {
                        println!("{}[{}]={}", indent(depth), attr.name, attr.value);
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    println!("{}-{}", indent(depth), name);
                }
                Ok(XmlEvent::StartDocument {
                    version,
                    encoding,
                    standalone,
                }) => {
                    if let Some(b) = standalone {
                        println!("has value {}", b);
                    }

                    println!("doc declaration");
                }
                Ok(XmlEvent::Characters(value)) => {
                    println!("{}VALUE: {}", indent(depth), value);
                }
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

        let val = ValueConverter::u32_to_bytes(String::from("1242342134343212341"));
        if val.is_ok() {
            println!("{:?}", val.unwrap());
        } else {
            let e = val.unwrap_err();
            eprintln!("error: {}", e);
        }
    }

    #[test]
    fn write_buffers() {
        let sb = DataBufferWriter::new_with_code_name("shift_jis");
        if sb.is_ok() {
            let g = sb.unwrap();
            let e = g.encoding();
            println!("{:?}", e.name());
        } else {
            println!("NOOOOO!!!");
        }
        // let dbw = DataBufferWriter::new();
        // let bytes = dbw.to_bytes();
        // let b = bytes.to_owned();
        // println!("{:?}", b);
    }
}
