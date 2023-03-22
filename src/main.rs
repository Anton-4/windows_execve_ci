use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

fn main() {
    let prog = CString::new("C:\\Windows\\System32\\cmd.exe").unwrap();
    let arg1 = CString::new("/C").unwrap();
    let arg2 = CString::new("echo").unwrap();
    let arg3 = CString::new("Hello, World!").unwrap();

    let argv = [
        prog.as_ptr(),
        arg1.as_ptr(),
        arg2.as_ptr(),
        arg3.as_ptr(),
        ptr::null::<c_char>(),
    ];

    unsafe {
        libc::execve(
            prog.as_ptr(),
            argv.as_ptr() as *const *const c_char,
            ptr::null::<*const c_char>(),
        );
    }
}