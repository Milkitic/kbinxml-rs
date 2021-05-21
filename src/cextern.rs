use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use stopwatch::Stopwatch;

use crate::{KBinWriter, traits::BinWriter, util::codec::Sixbit};
#[repr(C)]
pub struct Result {
    pub is_error: bool,
    pub arr_ptr: *const u8,
    pub arr_size: i32,
    pub error: *const c_char,
}

impl Result {
    pub fn error(str: &str) -> Result {
        Result {
            arr_ptr: get_zero_pointer(),
            arr_size: 0,
            is_error: true,
            error: get_cstring_without_collect(str),
        }
    }

    pub fn success(vec: &Vec<u8>) -> Result {
        Result {
            arr_ptr: vec.as_ptr(),
            arr_size: vec.len() as i32,
            is_error: false,
            error: get_cstring_without_collect(""),
        }
    }
}

fn get_cstring_without_collect(str: &str) -> *const c_char {
    let cs = CString::new(str).unwrap();
    let ptr = cs.as_ptr();
    std::mem::forget(cs);
    return ptr;
}

fn get_zero_pointer() -> *const u8 {
    0 as *const u8
}

// #[no_mangle]
// pub extern "C" fn test(xml_str: *const c_char, code_page: i32) -> Result {
//     let cstr = unsafe { CStr::from_ptr(xml_str) };
//     let result = cstr.to_str();
//     if result.is_err() {
//         return Result::error("Can not read string argument: xml_str.");
//     }

//     let r_str = result.unwrap();

//     let arr: Vec<u8> = vec![1, 1, 4, 5, 1, 4];
//     std::mem::forget(&arr);
//     return Result::success(&arr);
// }

#[no_mangle]
pub extern "C" fn sixcode_encode_test(str: *const c_char, count: i32) -> i64 {
    let result = unsafe { CStr::from_ptr(str).to_str() }.unwrap();
    let sw=Stopwatch::start_new();
    for i in 0..count {
        Sixbit::encode(&result.to_string()).unwrap();
    }
    sw.elapsed_ms()
}

#[no_mangle]
pub extern "C" fn encode_codepage(xml_str: *const c_char, code_page: i32) -> Result {
    let result = unsafe { CStr::from_ptr(xml_str).to_str() };
    if result.is_err() {
        return Result::error("Can not read string argument: xml_str.");
    }
    let xml = result.unwrap().to_string();
    // println!("XML length: {}", xml.len());

    let result = KBinWriter::new_with_code_page(code_page as usize);
    let message = match &result {
        Ok(_) => String::new(),
        Err(error) => {
            // println!("Problem creating KBinWriter: {}", error);
            format!("Problem creating KBinWriter: {}", error)
        }
    };
    if result.is_err() {
        return Result::error(&message);
    }
    let writer = result.unwrap();

    let result1 = writer.write(&xml);
    let message1 = match &result1 {
        Ok(_) => String::new(),
        Err(error) => {
            // println!("Problem while writing: {}", error);
            format!("Problem while writing: {}", error)
        }
    };
    if result1.is_err() {
        return Result::error(&message1);
    }

    let arr = result1.unwrap();
    let result = Result::success(&arr);
    std::mem::forget(arr);
    return result;
}

#[no_mangle]
pub extern "C" fn encode_codename(xml_str: *const c_char, code_name: *const c_char) -> Result {
    let result = unsafe { CStr::from_ptr(xml_str).to_str() };
    if result.is_err() {
        return Result::error("Can not read string argument: xml_str.");
    }
    let xml = result.unwrap().to_string();
    // println!("XML length: {}", xml.len());

    let result = unsafe { CStr::from_ptr(code_name).to_str() };
    if result.is_err() {
        return Result::error("Can not read string argument: code_name.");
    }
    let code_name = result.unwrap().to_string();

    let result = KBinWriter::new_with_code_name(&code_name);
    let message = match &result {
        Ok(_) => String::new(),
        Err(error) => {
            // println!("Problem creating KBinWriter: {}", error);
            format!("Problem creating KBinWriter: {}", error)
        }
    };
    if result.is_err() {
        return Result::error(&message);
    }
    let writer = result.unwrap();

    let result1 = writer.write(&xml);
    let message1 = match &result1 {
        Ok(_) => String::new(),
        Err(error) => {
            // println!("Problem while writing: {}", error);
            format!("Problem while writing: {}", error)
        }
    };
    if result1.is_err() {
        return Result::error(&message1);
    }

    let arr = result1.unwrap();
    let result = Result::success(&arr);
    std::mem::forget(arr);
    return result;
}
