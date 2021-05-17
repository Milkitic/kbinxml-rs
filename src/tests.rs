extern crate xml;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use xml::reader::{EventReader, XmlEvent};

    use crate::{traits::MyWriter, writer::KBinWriter};

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
        let writer = KBinWriter::new();
        writer.write("<test></test>");
    }
}
