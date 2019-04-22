use winapi::um::winnt::HANDLE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::fileapi::GetFinalPathNameByHandleW;
use winapi::shared::ntdef::LPWSTR;
use std::ffi::OsString;

use std::os::windows::prelude::*;
use std::io::stdout;
use std::io::Write;
use std::convert::TryInto;


fn main() {
    unsafe {
        let handle: HANDLE = GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);
        let _name = {
            let mut buf = [0u16; 2048];


            let name_len = GetFinalPathNameByHandleW(
                handle,
                &mut buf as *mut _ as LPWSTR,
                (buf.len() - 1).try_into().expect("Number too big"),
                0
            );

            if name_len as usize > buf.len() - 1 {
                //to big file length, bigger then buffer, we son't handle this, that won't be ttys we are looking for.
                return;
            }


            let str = OsString::from_wide(&buf[0..(name_len as usize)]);

            str.to_string_lossy().into_owned()
        };


        println!("STDOUT file: {}", _name);
        let _ =stdout().flush();
    }
}
