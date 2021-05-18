pub mod buffer;

extern crate stringreader;
extern crate xml;

use std::{collections::BTreeMap, io::BufReader};

use stringreader::StringReader;
use xml::reader::{EventReader, XmlEvent};

use crate::{
    traits::{BigEndianBinaryWrite, BinWriter},
    types::BoxResult,
};

use self::buffer::{DataBufferWriter, NodeBufferWriter};

pub struct KBinWriter<'a> {
    node_writer: NodeBufferWriter,
    data_writer: DataBufferWriter<'a>,
}

impl<'a> KBinWriter<'a> {
    pub fn new_with_code_name(code_name: &str) -> BoxResult<Self> {
        let data_writer = DataBufferWriter::new_with_code_name(code_name)?;
        Ok(Self {
            node_writer: NodeBufferWriter::new(),
            data_writer: data_writer,
        })
    }

    pub fn new_with_code_page(code_page: usize) -> BoxResult<Self> {
        let data_writer = DataBufferWriter::new_with_code_page(code_page)?;
        Ok(Self {
            node_writer: NodeBufferWriter::new(),
            data_writer: data_writer,
        })
    }
}

impl BinWriter for KBinWriter<'_> {
    fn write(mut self, content: &str) -> BoxResult<Vec<u8>> {
        let sr = StringReader::new(content);
        let buf_reader = BufReader::new(sr);
        let mut conf = xml::reader::ParserConfig::new();
        conf.ignore_comments = true;

        let parser = EventReader::new_with_config(buf_reader, conf);

        let mut holding_attrs: BTreeMap<String, String> = BTreeMap::new();
        let mut holding_value: String = String::new();
        let mut type_str: String = String::new();
        let mut size_str: String = String::new();
        let mut typeid: u8 = 0;

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    ensure_holding(
                        &mut type_str,
                        &mut size_str,
                        &mut holding_value,
                        &mut holding_attrs,
                        &mut typeid,
                    );

                    for attr in attributes {
                        let name = attr.name.local_name;
                        let value = attr.value;
                        if name == "__type" {
                            type_str = value;
                        } else if name == "__count" {
                            size_str = value;
                        } else {
                            holding_attrs.entry(name).or_insert(value);
                        }
                    }

                    if type_str.is_empty() {
                        self.node_writer.write_u8(1)?;
                        self.node_writer.write_string(&name.local_name)?;
                        // _nodeBuffer.WriteU8(1);
                        // _nodeBuffer.WriteString(reader.Name);
                    } else {
                        // typeid = TypeDictionary.ReverseTypeMap[typeStr];
                        // if (sizeStr != null)
                        //     _nodeBuffer.WriteU8((byte)(typeid | 0x40));
                        // else
                        //     _nodeBuffer.WriteU8(typeid);

                        // _nodeBuffer.WriteString(reader.Name);
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    ensure_holding(
                        &mut type_str,
                        &mut size_str,
                        &mut holding_value,
                        &mut holding_attrs,
                        &mut typeid,
                    );
                    // _nodeBuffer.WriteU8(0xFE);
                }
                Ok(XmlEvent::Characters(value)) => {
                    holding_value = value;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        ensure_holding(
            &mut type_str,
            &mut size_str,
            &mut holding_value,
            &mut holding_attrs,
            &mut typeid,
        );

        // _nodeBuffer.WriteU8(255);
        // _nodeBuffer.Pad();
        // _dataBuffer.Pad();

        // output.WriteU8(0xA0); //Magic
        // output.WriteU8(0x42); //Compression flag
        // output.WriteU8(EncodingDictionary.ReverseEncodingMap[_encoding]);
        // output.WriteU8((byte)~EncodingDictionary.ReverseEncodingMap[_encoding]);

        // //Write node buffer length and contents.
        // var buffer = _nodeBuffer.ToArray();
        // output.WriteS32(buffer.Length);
        // output.WriteBytes(buffer);

        // //Write data buffer length and contents.
        // var array = _dataBuffer.ToArray();
        // output.WriteS32(array.Length);
        // output.WriteBytes(array);

        // return output.ToArray();

        Ok(Vec::new())
    }
}

fn ensure_holding(
    type_str: &mut String,
    size_str: &mut String,
    holding_value: &mut String,
    holding_attrs: &mut BTreeMap<String, String>,
    typeid: &mut u8,
) -> () {
    if !type_str.is_empty() {
        if type_str == "str" {
            // _dataBuffer.WriteString(holdingValue);
        } else if type_str == "bin" {
            // _dataBuffer.WriteBinary(holdingValue);
        } else {
        }

        type_str.clear();
        size_str.clear();
        holding_value.clear();
        *typeid = 0;
    }

    if holding_attrs.len() > 0 {
        for tuple in holding_attrs.iter() {
            // _nodeBuffer.WriteU8(0x2E);
            // _nodeBuffer.WriteString(attribute.Key);
            // _dataBuffer.WriteString(attribute.Value);
        }

        holding_attrs.clear();
    }
}
