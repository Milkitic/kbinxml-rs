pub mod buffer;

extern crate stringreader;
extern crate xml;

use std::{collections::BTreeMap, io::BufReader};

use stringreader::StringReader;
use xml::reader::{EventReader, XmlEvent};

use crate::{
    datamapping::{EncodingDictionary, TypeDictionary},
    traits::{BigEndianBinaryWrite, BinWriter},
    types::BoxResult,
};

use self::buffer::{BigEndianBinaryWriter, DataBufferWriter, NodeBufferWriter};

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

    fn ensure_holding(
        &mut self,
        type_str: &mut String,
        size_str: &mut String,
        holding_value: &mut String,
        holding_attrs: &mut BTreeMap<String, String>,
        typeid: &mut u8,
    ) -> BoxResult<()> {
        if !type_str.is_empty() {
            if type_str == "str" {
                self.data_writer.write_string(holding_value)?;
            } else if type_str == "bin" {
                self.data_writer.write_binary(holding_value)?;
            } else {
            }

            type_str.clear();
            size_str.clear();
            holding_value.clear();
            *typeid = 0;
        }

        if holding_attrs.len() > 0 {
            for tuple in holding_attrs.iter() {
                self.node_writer.write_u8(0x2E)?;
                self.node_writer.write_string(tuple.0)?;
                self.data_writer.write_string(tuple.1)?;
            }

            holding_attrs.clear();
        }

        Ok(())
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
                    self.ensure_holding(
                        &mut type_str,
                        &mut size_str,
                        &mut holding_value,
                        &mut holding_attrs,
                        &mut typeid,
                    )?;

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
                    } else {
                        typeid = TypeDictionary::get_node_flag(&type_str)? as u8;

                        if !size_str.is_empty() {
                            self.node_writer.write_u8(typeid | 0x40)?;
                        } else {
                            self.node_writer.write_u8(typeid)?;
                        }

                        self.node_writer.write_string(&name.local_name)?;
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    self.ensure_holding(
                        &mut type_str,
                        &mut size_str,
                        &mut holding_value,
                        &mut holding_attrs,
                        &mut typeid,
                    )?;
                    self.node_writer.write_u8(0xFE)?;
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

        self.ensure_holding(
            &mut type_str,
            &mut size_str,
            &mut holding_value,
            &mut holding_attrs,
            &mut typeid,
        )?;

        self.node_writer.write_u8(0xFF)?;
        self.node_writer.pad()?;
        self.data_writer.pad()?;

        let mut output = BigEndianBinaryWriter::new();
        output.write_u8(0xA0)?; // Magic
        output.write_u8(0x42)?; // Compression flag
        let value = EncodingDictionary::get_encode_flag(self.data_writer.encoding())?;
        output.write_u8(value as u8)?;
        output.write_u8((!value) as u8)?;

        // Write node buffer length and contents.
        let buffer = self.node_writer.to_bytes();
        output.write_s32(buffer.len() as i32)?;
        output.write_bytes(buffer)?;

        //Write data buffer length and contents.
        let array = self.data_writer.to_bytes();
        output.write_s32(array.len() as i32)?;
        output.write_bytes(array)?;

        Ok(output.to_bytes().to_owned())
    }
}
