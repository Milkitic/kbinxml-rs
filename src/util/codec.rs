use crate::types::BoxResult;
use phf::{phf_map, phf_set, Map, Set};

pub struct Sixbit {}

impl Sixbit {
    const CHARSET: Set<char> = phf_set! {
        '0', '1', '2', '3', '4', '5', '6', '7',
        '8', '9', ':', 'A', 'B', 'C', 'D', 'E',
        'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
        'V', 'W', 'X', 'Y', 'Z', '_', 'a', 'b',
        'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    };

    const CHARSET_REVERSE: Map<char, u8> = phf_map! {
        '0'=>0 , '1'=>1 , '2'=>2 , '3'=>3 , '4'=>4 , '5'=>5 , '6'=>6 , '7'=>7 ,
        '8'=>8 , '9'=>9 , ':'=>10, 'A'=>11, 'B'=>12, 'C'=>13, 'D'=>14, 'E'=>15,
        'F'=>16, 'G'=>17, 'H'=>18, 'I'=>19, 'J'=>20, 'K'=>21, 'L'=>22, 'M'=>23,
        'N'=>24, 'O'=>25, 'P'=>26, 'Q'=>27, 'R'=>28, 'S'=>29, 'T'=>30, 'U'=>31,
        'V'=>32, 'W'=>33, 'X'=>34, 'Y'=>35, 'Z'=>36, '_'=>37, 'a'=>38, 'b'=>39,
        'c'=>40, 'd'=>41, 'e'=>42, 'f'=>43, 'g'=>44, 'h'=>45, 'i'=>46, 'j'=>47,
        'k'=>48, 'l'=>49, 'm'=>50, 'n'=>51, 'o'=>52, 'p'=>53, 'q'=>54, 'r'=>55,
        's'=>56, 't'=>57, 'u'=>58, 'v'=>59, 'w'=>60, 'x'=>61, 'y'=>62, 'z'=>63,
    };

    pub fn encode(input: &str) -> BoxResult<Vec<u8>> {
       for i in 0..input.len() as u32 {
           let g=input[i];
       }
        // let char_arr: Vec<char> = input.chars().collect();
        // char_arr.len();
        // let mut buffer: Vec<u8> = vec![0; char_arr.len()];
        // for i in 0..input.len() {
        //     let c = char_arr[i];
        //     buffer[i] = Sixbit::CHARSET_REVERSE[&c];
        // }
        // let buffer : Vec<u8> = input.chars().map(|c| Sixbit::CHARSET_REVERSE[&c]).collect();
        // let buffer : Vec<u8> = input.chars().map(|c| 0).collect();

        // let length = (buffer.len() as f64 * 6.0 / 8.0).ceil() as i32;
        // let mut output: Vec<u8> = vec![0; length as usize];
        let mut output: Vec<u8> = vec![0;0];

        // for i in 0..(buffer.len() * 6) {
        //     output[i / 8] =
        //         (output[i / 8] | ((buffer[i / 6] >> (5 - (i % 6)) & 1) << (7 - (i % 8)))) as u8;
        // }

        Ok(output)
    }
}
