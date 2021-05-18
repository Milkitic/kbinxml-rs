use std::io::Error;

use encoding::Encoding;

use crate::types::{BoxResult, ByteToStringFunc, StringToByteFunc};

use self::converter::ValueConverter;

pub(crate) mod converter;

pub struct NodeType {
    size: i32,
    count: i32,
    name: String,
    get_bytes: StringToByteFunc,
    // get_string: ByteToStringFunc,
}

impl NodeType {
    pub fn new(
        size: i32,
        count: i32,
        name: String,
        get_bytes: StringToByteFunc,
        // get_string: ByteToStringFunc,
    ) -> Self {
        Self {
            size,
            count,
            name,
            get_bytes,
            // get_string,
        }
    }
}

pub struct EncodingDictionary {}

impl EncodingDictionary {
    pub fn get_encode_flag(encoding: &dyn Encoding) -> BoxResult<i32> {
        if encoding.whatwg_name().is_none() {
            return Err(Box::new(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unsupported encoding: {}", encoding.name()),
            )));
        }

        let name = encoding.whatwg_name().unwrap();
        match name {
            "ascii" => Ok(0x20),
            "windows-1252" => Ok(0x40),
            "euc-jp" => Ok(0x60),
            "shift_jis" => Ok(0x80),
            "utf-8" => Ok(0xA0),
            _ => Err(Box::new(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unsupported encoding: {}", name),
            ))),
        }
    }
}

pub struct TypeDictionary {}

impl TypeDictionary {
    pub fn get_node_flag(t: &String) -> BoxResult<i32>{
        match t.as_str(){
            "s8"    =>Ok(2 ),
            "u8"    =>Ok(3 ),
            "s16"   =>Ok(4 ),
            "u16"   =>Ok(5 ),
            "s32"   =>Ok(6 ),
            "u32"   =>Ok(7 ),
            "s64"   =>Ok(8 ),
            "u64"   =>Ok(9 ),
            "bin"   =>Ok(10),
            "str"   =>Ok(11),
            "ip4"   =>Ok(12),
            "time"  =>Ok(13),
            "float" =>Ok(14),
            "double"=>Ok(15),
            "2s8"   =>Ok(16),
            "2u8"   =>Ok(17),
            "2s16"  =>Ok(18),
            "2u16"  =>Ok(19),
            "2s32"  =>Ok(20),
            "2u32"  =>Ok(21),
            "vs64"  =>Ok(22),
            "vu64"  =>Ok(23),
            "2f"    =>Ok(24),
            "vd"    =>Ok(25),
            "3s8"   =>Ok(26),
            "3u8"   =>Ok(27),
            "3s16"  =>Ok(28),
            "3u16"  =>Ok(29),
            "3s32"  =>Ok(30),
            "3u32"  =>Ok(31),
            "3s64"  =>Ok(32),
            "3u64"  =>Ok(33),
            "3f"    =>Ok(34),
            "3d"    =>Ok(35),
            "4s8"   =>Ok(36),
            "4u8"   =>Ok(37),
            "4s16"  =>Ok(38),
            "4u16"  =>Ok(39),
            "vs32"  =>Ok(40),
            "vu32"  =>Ok(41),
            "4s64"  =>Ok(42),
            "4u64"  =>Ok(43),
            "vf"    =>Ok(44),
            "4d"    =>Ok(45),
            "vs8"   =>Ok(48),
            "vu8"   =>Ok(49),
            "vs16"  =>Ok(50),
            "vu16"  =>Ok(51),
            "bool"  =>Ok(52),
            "2b"    =>Ok(53),
            "3b"    =>Ok(54),
            "4b"    =>Ok(55),
            "vb"    =>Ok(56),
            _ => Err(Box::new(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid type: {}", t),
            ))),
        }
    }
    
    pub fn get_node_type(flag: i32) -> BoxResult<NodeType> {
        match flag {
            2  => Ok(NodeType::new(1, 1,  "s8"    .to_string(),|str|{ Ok(ValueConverter::s8_to_bytes    (str)?.to_vec()) })),
            3  => Ok(NodeType::new(1, 1,  "u8"    .to_string(),|str|{ Ok(ValueConverter::u8_to_bytes    (str)?.to_vec()) })),
            4  => Ok(NodeType::new(2, 1,  "s16"   .to_string(),|str|{ Ok(ValueConverter::s16_to_bytes   (str)?.to_vec()) })),
            5  => Ok(NodeType::new(2, 1,  "u16"   .to_string(),|str|{ Ok(ValueConverter::u16_to_bytes   (str)?.to_vec()) })),
            6  => Ok(NodeType::new(4, 1,  "s32"   .to_string(),|str|{ Ok(ValueConverter::s32_to_bytes   (str)?.to_vec()) })),
            7  => Ok(NodeType::new(4, 1,  "u32"   .to_string(),|str|{ Ok(ValueConverter::u32_to_bytes   (str)?.to_vec()) })),
            8  => Ok(NodeType::new(8, 1,  "s64"   .to_string(),|str|{ Ok(ValueConverter::s64_to_bytes   (str)?.to_vec()) })),
            9  => Ok(NodeType::new(8, 1,  "u64"   .to_string(),|str|{ Ok(ValueConverter::u64_to_bytes   (str)?.to_vec()) })),
            10 => Ok(NodeType::new(0, 0,  "bin"   .to_string(),|str|{ Ok(ValueConverter::default        (str)?.to_vec()) })),
            11 => Ok(NodeType::new(0, 0,  "str"   .to_string(),|str|{ Ok(ValueConverter::default        (str)?.to_vec()) })),
            12 => Ok(NodeType::new(4, 1,  "ip4"   .to_string(),|str|{ Ok(ValueConverter::ip4_to_bytes   (str)?.to_vec()) })),
            13 => Ok(NodeType::new(4, 1,  "time"  .to_string(),|str|{ Ok(ValueConverter::u32_to_bytes   (str)?.to_vec()) })),
            14 => Ok(NodeType::new(4, 1,  "float". to_string(),|str|{ Ok(ValueConverter::single_to_bytes(str)?.to_vec()) })),
            15 => Ok(NodeType::new(8, 1,  "double".to_string(),|str|{ Ok(ValueConverter::double_to_bytes(str)?.to_vec()) })),
            16 => Ok(NodeType::new(1, 2,  "2s8"   .to_string(),|str|{ Ok(ValueConverter::s8_to_bytes    (str)?.to_vec()) })),
            17 => Ok(NodeType::new(1, 2,  "2u8"   .to_string(),|str|{ Ok(ValueConverter::u8_to_bytes    (str)?.to_vec()) })),
            18 => Ok(NodeType::new(2, 2,  "2s16"  .to_string(),|str|{ Ok(ValueConverter::s16_to_bytes   (str)?.to_vec()) })),
            19 => Ok(NodeType::new(2, 2,  "2u16"  .to_string(),|str|{ Ok(ValueConverter::u16_to_bytes   (str)?.to_vec()) })),
            20 => Ok(NodeType::new(4, 2,  "2s32"  .to_string(),|str|{ Ok(ValueConverter::s32_to_bytes   (str)?.to_vec()) })),
            21 => Ok(NodeType::new(4, 2,  "2u32"  .to_string(),|str|{ Ok(ValueConverter::u32_to_bytes   (str)?.to_vec()) })),
            22 => Ok(NodeType::new(8, 2,  "vs64"  .to_string(),|str|{ Ok(ValueConverter::s64_to_bytes   (str)?.to_vec()) })),
            23 => Ok(NodeType::new(8, 2,  "vu64"  .to_string(),|str|{ Ok(ValueConverter::u64_to_bytes   (str)?.to_vec()) })),
            24 => Ok(NodeType::new(4, 2,  "2f"    .to_string(),|str|{ Ok(ValueConverter::single_to_bytes(str)?.to_vec()) })),
            25 => Ok(NodeType::new(8, 2,  "vd"    .to_string(),|str|{ Ok(ValueConverter::double_to_bytes(str)?.to_vec()) })),
            26 => Ok(NodeType::new(1, 3,  "3s8"   .to_string(),|str|{ Ok(ValueConverter::s8_to_bytes    (str)?.to_vec()) })),
            27 => Ok(NodeType::new(1, 3,  "3u8"   .to_string(),|str|{ Ok(ValueConverter::u8_to_bytes    (str)?.to_vec()) })),
            28 => Ok(NodeType::new(2, 3,  "3s16"  .to_string(),|str|{ Ok(ValueConverter::s16_to_bytes   (str)?.to_vec()) })),
            29 => Ok(NodeType::new(2, 3,  "3u16"  .to_string(),|str|{ Ok(ValueConverter::u16_to_bytes   (str)?.to_vec()) })),
            30 => Ok(NodeType::new(4, 3,  "3s32"  .to_string(),|str|{ Ok(ValueConverter::s32_to_bytes   (str)?.to_vec()) })),
            31 => Ok(NodeType::new(4, 3,  "3u32"  .to_string(),|str|{ Ok(ValueConverter::u32_to_bytes   (str)?.to_vec()) })),
            32 => Ok(NodeType::new(8, 3,  "3s64"  .to_string(),|str|{ Ok(ValueConverter::s64_to_bytes   (str)?.to_vec()) })),
            33 => Ok(NodeType::new(8, 3,  "3u64"  .to_string(),|str|{ Ok(ValueConverter::u64_to_bytes   (str)?.to_vec()) })),
            34 => Ok(NodeType::new(4, 3,  "3f"    .to_string(),|str|{ Ok(ValueConverter::single_to_bytes(str)?.to_vec()) })),
            35 => Ok(NodeType::new(8, 3,  "3d"    .to_string(),|str|{ Ok(ValueConverter::double_to_bytes(str)?.to_vec()) })),
            36 => Ok(NodeType::new(1, 4,  "4s8"   .to_string(),|str|{ Ok(ValueConverter::s8_to_bytes    (str)?.to_vec()) })),
            37 => Ok(NodeType::new(1, 4,  "4u8"   .to_string(),|str|{ Ok(ValueConverter::u8_to_bytes    (str)?.to_vec()) })),
            38 => Ok(NodeType::new(2, 4,  "4s16"  .to_string(),|str|{ Ok(ValueConverter::s16_to_bytes   (str)?.to_vec()) })),
            39 => Ok(NodeType::new(2, 4,  "4u16"  .to_string(),|str|{ Ok(ValueConverter::u16_to_bytes   (str)?.to_vec()) })),
            40 => Ok(NodeType::new(4, 4,  "vs32"  .to_string(),|str|{ Ok(ValueConverter::s32_to_bytes   (str)?.to_vec()) })),
            41 => Ok(NodeType::new(4, 4,  "vu32"  .to_string(),|str|{ Ok(ValueConverter::u32_to_bytes   (str)?.to_vec()) })),
            42 => Ok(NodeType::new(8, 4,  "4s64"  .to_string(),|str|{ Ok(ValueConverter::s64_to_bytes   (str)?.to_vec()) })),
            43 => Ok(NodeType::new(8, 4,  "4u64"  .to_string(),|str|{ Ok(ValueConverter::u64_to_bytes   (str)?.to_vec()) })),
            44 => Ok(NodeType::new(4, 4,  "vf"    .to_string(),|str|{ Ok(ValueConverter::single_to_bytes(str)?.to_vec()) })),
            45 => Ok(NodeType::new(8, 4,  "4d"    .to_string(),|str|{ Ok(ValueConverter::double_to_bytes(str)?.to_vec()) })),
            48 => Ok(NodeType::new(1, 16, "vs8"   .to_string(),|str|{ Ok(ValueConverter::s8_to_bytes    (str)?.to_vec()) })),
            49 => Ok(NodeType::new(1, 16, "vu8"   .to_string(),|str|{ Ok(ValueConverter::u8_to_bytes    (str)?.to_vec()) })),
            50 => Ok(NodeType::new(2, 8,  "vs16"  .to_string(),|str|{ Ok(ValueConverter::s16_to_bytes   (str)?.to_vec()) })),
            51 => Ok(NodeType::new(2, 8,  "vu16"  .to_string(),|str|{ Ok(ValueConverter::u16_to_bytes   (str)?.to_vec()) })),
            52 => Ok(NodeType::new(1, 1,  "bool"  .to_string(),|str|{ Ok(ValueConverter::u8_to_bytes    (str)?.to_vec()) })),
            53 => Ok(NodeType::new(1, 2,  "2b"    .to_string(),|str|{ Ok(ValueConverter::u8_to_bytes    (str)?.to_vec()) })),
            54 => Ok(NodeType::new(1, 3,  "3b"    .to_string(),|str|{ Ok(ValueConverter::u8_to_bytes    (str)?.to_vec()) })),
            55 => Ok(NodeType::new(1, 4,  "4b"    .to_string(),|str|{ Ok(ValueConverter::u8_to_bytes    (str)?.to_vec()) })),
            56 => Ok(NodeType::new(1, 16, "vb"    .to_string(),|str|{ Ok(ValueConverter::u8_to_bytes    (str)?.to_vec()) })),
            _ => Err(Box::new(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid flag: {}", flag),
            ))),
        }
    }
}
