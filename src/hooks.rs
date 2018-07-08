use std::ffi::CString;
use user32::*;
use kernel32::*;
use ws2_32;
use winapi::*;
use winapi::minwindef::*;
use winapi::winnt::*;
use helpers;
use std;
use std::num::Wrapping;

static mut RECVADDR: *mut c_void = std::ptr::null_mut();

pub unsafe fn place_jmp(address: DWORD, jumpto: DWORD, length: DWORD) {
    let dw_old_protect: DWORD = 0;
    let mut dw_bkup: DWORD = 0;
    let mut dw_rel_addr: DWORD = 0;

    VirtualProtect(address as *mut c_void, length, PAGE_EXECUTE_READWRITE, &mut dw_rel_addr);

    dw_rel_addr = (jumpto.wrapping_sub(address)).wrapping_sub(0x5);

    {
        helpers::print_hex("jumpto", jumpto);
        helpers::print_hex("address", address);
        helpers::print_hex("Rel addy", dw_rel_addr);
    }

    *(address as *mut i32) = 0xE9;
    *((address + 0x1) as *mut DWORD) = dw_rel_addr;

    for x in 0x5..length {
        *((address + x) as *mut DWORD) = 0x90;
    }

    VirtualProtect(address as *mut c_void, length, dw_old_protect, &mut dw_bkup);

}


#[naked]
pub unsafe extern "system" fn real_recv(_s: SOCKET, _buf: *mut c_char, _len: c_int, _flags: c_int) -> i32{
    asm!(
        "mov edi, edi;
        push ebp;
        mov ebp, esp;
        mov eax, eax;
        add eax, 5;
        jmp eax;"
        :
        : "-{eax}"(RECVADDR)
        :
        :"intel"
    );


    return 0;
}


pub unsafe fn get_recv() -> DWORD {
    RECVADDR = GetProcAddress(GetModuleHandleA(helpers::c_str("ws2_32").as_ptr()), helpers::c_str("recv").as_ptr()) as *mut c_void;
    return RECVADDR as DWORD;
}

pub unsafe extern "system" fn hook_recv(s: SOCKET, buf: *mut c_char, len: c_int, flags: c_int) -> c_int {
    println!("RECV!");
    return real_recv(s, buf,len, flags);
}