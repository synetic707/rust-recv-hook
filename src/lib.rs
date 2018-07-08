#![feature(asm, naked_functions)]
#![allow(unused_imports)]

extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate ws2_32;

use std::ffi::CString;
use user32::*;
use kernel32::*;
use ws2_32::*;
use winapi::*;
use winapi::minwindef::DWORD;
use std::thread;
use winapi::minwinbase::LPTHREAD_START_ROUTINE;
use std::option::Option;

mod helpers;
mod hooks;

unsafe extern "system" fn keep_alive(_lpthreadparameter: winapi::LPVOID) -> DWORD {
    loop {
        thread::sleep(std::time::Duration::from_millis(250));
    }

    return 0;
}


#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, reserved: LPVOID) -> BOOL
{
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => unsafe {
            CreateThread(std::ptr::null_mut(), 0, Some(keep_alive), std::ptr::null_mut(), 0, std::ptr::null_mut());
            init();
        },
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    return TRUE;
}


unsafe fn init() {
    kernel32::AllocConsole();

    helpers::print_hex("Real Recv", hooks::get_recv());
    helpers::print_hex("Fake Recv", hooks::hook_recv as DWORD);
    hooks::place_jmp(hooks::get_recv(), hooks::hook_recv as DWORD, 5);

}
