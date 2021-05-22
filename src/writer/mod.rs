pub mod buffer;

extern crate stringreader;

use std::{collections::BTreeMap, io::Error};

use quick_xml::{events::Event, Reader};
use stopwatch::Stopwatch;

use crate::{
    traits::{BigEndianBinaryWrite, BinWriter},
    types::BoxResult,
    util::{dictionary::EncodingDictionary, TypeDictionary},
};

use self::buffer::{BigEndianBinaryWriter, DataBufferWriter, NodeBufferWriter};

pub struct KBinWriter<'a> {
    node_writer: NodeBufferWriter<'a>,
    data_writer: DataBufferWriter<'a>,
}

impl<'a> KBinWriter<'a> {
    pub fn new_with_code_name(code_name: &str) -> BoxResult<Self> {
        let data_writer = DataBufferWriter::new_with_code_name(code_name)?;
        let node_writer = NodeBufferWriter::new_with_code_name(true, code_name)?;
        Ok(Self {
            node_writer: node_writer,
            data_writer: data_writer,
        })
    }

    pub fn new_with_code_page(code_page: usize) -> BoxResult<Self> {
        let data_writer = DataBufferWriter::new_with_code_page(code_page)?;
        let node_writer = NodeBufferWriter::new_with_code_page(true, code_page)?;
        Ok(Self {
            node_writer: node_writer,
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
                let t = TypeDictionary::get_node_type(*typeid as i32)?;
                let split = holding_value.split(' ');
                let mut size = t.size().clone(); // let mut size = *t.size(); ?
                if !size_str.is_empty() {
                    size = size * size_str.parse::<i32>()?;
                    self.data_writer.write_u32(size as u32)?;
                }

                let mut arr = vec![0; 0];

                let mut i: i32 = 0;
                for s in split {
                    if i == size {
                        break;
                    }

                    let mut buffer = t.get_bytes()(s)?;
                    arr.append(&mut buffer);

                    i += t.size();
                }

                self.data_writer.write_bytes(&arr)?;
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

    // fn get_string(raw: &[u8], encoding: &dyn Encoding) -> BoxResult<String> {
    //     Ok(encoding.decode(raw, encoding::DecoderTrap::Replace)?)
    // }
}

impl BinWriter for KBinWriter<'_> {
    fn write(mut self, content: &str) -> BoxResult<Vec<u8>> {
        // let sw = Stopwatch::start_new();
        let mut reader = Reader::from_str(content);
        let mut buf = Vec::new();
        reader.trim_text(true);
        reader.check_comments(false);

        // let mut encoding: &(dyn Encoding + Send + Sync) =
        //     encoding::label::encoding_from_whatwg_label("utf-8").unwrap();

        let mut holding_attrs: BTreeMap<String, String> = BTreeMap::new();
        let mut holding_value: String = String::new();
        let mut type_str: String = String::new();
        let mut size_str: String = String::new();
        let mut typeid: u8 = 0;

        // let xmlSw = Stopwatch::start_new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    // let name = KBinWriter::<'_>::get_string(&e.local_name(), encoding).unwrap();
                    let name = reader.decode(&e.local_name())?;
                    self.ensure_holding(
                        &mut type_str,
                        &mut size_str,
                        &mut holding_value,
                        &mut holding_attrs,
                        &mut typeid,
                    )?;

                    for attr in e.attributes() {
                        let val = attr?;
                        let attr_name = reader.decode(&val.key)?;
                        let attr_val = reader.decode(&val.value)?;
                        // let attr_name = KBinWriter::<'_>::get_string(val.key, encoding).unwrap();
                        // let attr_val = KBinWriter::<'_>::get_string(&val.value, encoding).unwrap();
                        if attr_name == "__type" {
                            type_str = attr_val.to_string();
                        } else if attr_name == "__count" {
                            size_str = attr_val.to_string();
                        } else {
                            holding_attrs
                                .entry(attr_name.to_string())
                                .or_insert(attr_val.to_string());
                        }
                    }

                    if type_str.is_empty() {
                        self.node_writer.write_u8(1)?;
                        self.node_writer.write_string(&name)?;
                    } else {
                        typeid = TypeDictionary::get_node_flag(&type_str)? as u8;

                        if !size_str.is_empty() {
                            self.node_writer.write_u8(typeid | 0x40)?;
                        } else {
                            self.node_writer.write_u8(typeid)?;
                        }

                        self.node_writer.write_string(&name)?;
                    }
                }
                Ok(Event::End(_e)) => {
                    self.ensure_holding(
                        &mut type_str,
                        &mut size_str,
                        &mut holding_value,
                        &mut holding_attrs,
                        &mut typeid,
                    )?;
                    self.node_writer.write_u8(0xFE)?;
                }
                Ok(Event::Text(e)) => {
                    // let u8 = e.unescaped().unwrap();
                    // let value = KBinWriter::<'_>::get_string(&u8, encoding).unwrap();
                    let u8 = e.unescaped()?;
                    let value = reader.decode(&u8)?;
                    holding_value = value.to_string();
                }
                Ok(Event::Eof) => {
                    // self.ensure_holding(
                    //     &mut type_str,
                    //     &mut size_str,
                    //     &mut holding_value,
                    //     &mut holding_attrs,
                    //     &mut typeid,
                    // )?;

                    break;
                } // exits the loop when reaching end of file
                Err(e) => {
                    return Err(Box::new(Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Error while reading XML: {}", e),
                    )));
                }
                _ => {}
            }
        }

        // println!("XML iter: {}ms.", xmlSw.elapsed_ms());

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

        let vec = output.to_bytes().to_owned();
        // println!("KBin write: {}ms.", sw.elapsed_ms());
        Ok(vec)
    }
}
