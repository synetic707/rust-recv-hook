use std::ffi::CString;
use user32::*;
use kernel32::*;
use ws2_32::*;
use winapi::minwindef::*;

pub fn c_str(text: &str) -> CString {
    return CString::new(text).unwrap();
}


pub fn print_hex(text: &str, hex: DWORD) {
    let print = format!("{} address: 0x{:X?}", text, hex);
    println!("{}", print);
}
