use std::ffi::{CString, OsStr};
use std::os::raw::c_char;
use std::ptr;

use bumpalo::Bump;
use bumpalo::collections::CollectIn;

fn main() {
    let prog = CString::new("C:\\Users\\anton\\Desktop\\roc_app_binary.exe").unwrap();

    let argv = [
        prog.as_ptr(),
        ptr::null::<c_char>(),
    ];

    /*let env_vars = [
        "=::=::\\",
        "ALLUSERSPROFILE=C:\\ProgramData",
        "APPDATA=C:\\Users\\anton\\AppData\\Roaming",
        "CommonProgramFiles=C:\\Program Files\\Common Files",
        "CommonProgramFiles(x86)=C:\\Program Files (x86)\\Common Files",
        "CommonProgramW6432=C:\\Program Files\\Common Files",
        "COMPUTERNAME=DESKTOP-8CPMMTF",
        "ComSpec=C:\\Windows\\system32\\cmd.exe",
        "DriverData=C:\\Windows\\System32\\Drivers\\DriverData",
        "HOMEDRIVE=C:",
        "HOMEPATH=\\Users\\anton",
        "LLVM_SYS_130_PREFIX=C:\\Users\\anton\\Downloads\\LLVM-13.0.0-win64",
        "LOCALAPPDATA=C:\\Users\\anton\\AppData\\Local",
        "LOGONSERVER=\\\\DESKTOP-8CPMMTF",
        "NUMBER_OF_PROCESSORS=16",
        "OneDrive=C:\\Users\\anton\\OneDrive",
        "OneDriveConsumer=C:\\Users\\anton\\OneDrive",
        "OS=Windows_NT",
        "Path=C:\\Windows\\system32;C:\\Windows;C:\\Windows\\System32\\Wbem;C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\;C:\\Windows\\System32\\OpenSSH\\;C:\\Program Files\\dotnet\\;C:\\Program Files\\Git\\cmd;C:\\Program Files\\LLVM\\bin;C:\\Users\\anton\\Downloads\\zig-windows-x86_64-0.9.1;C:\\Users\\anton\\Downloads\\zig-windows-x86_64-0.9.1;C:\\Program Files\\NVIDIA Corporation\\NVIDIA NvDLISR;C:\\Program Files (x86)\\NVIDIA Corporation\\PhysX\\Common;C:\\Users\\anton\\AppData\\Local\\Programs\\Python\\Python310\\Scripts\\;C:\\Users\\anton\\AppData\\Local\\Programs\\Python\\Python310\\;C:\\Users\\anton\\.cargo\\bin;C:\\Users\\anton\\AppData\\Local\\Microsoft\\WindowsApps;C:\\Users\\anton\\.dotnet\\tools;C:\\Users\\anton\\AppData\\Local\\Programs\\Microsoft VS Code\\bin;C:\\Users\\anton\\Downloads\\zig-windows-x86_64-0.9.1\\zig-windows-x86_64-0.9.1;C:\\Program Files (x86)\\Dr. Memory\\bin\\",
        "PATHEXT=.COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH;.MSC;.CPL",
        "PROCESSOR_ARCHITECTURE=AMD64",
        "PROCESSOR_IDENTIFIER=AMD64 Family 25 Model 33 Stepping 0, AuthenticAMD",
        "PROCESSOR_LEVEL=25",
        "PROCESSOR_REVISION=2100",
        "ProgramData=C:\\ProgramData",
        "ProgramFiles=C:\\Program Files",
        "ProgramFiles(x86)=C:\\Program Files (x86)",
        "ProgramW6432=C:\\Program Files",
        "PSModulePath=C:\\Users\\anton\\Documents\\WindowsPowerShell\\Modules;C:\\Program Files\\WindowsPowerShell\\Modules;C:\\Windows\\system32\\WindowsPowerShell\\v1.0\\Modules",
        "PUBLIC=C:\\Users\\Public",
        "SESSIONNAME=Console",
        "SystemDrive=C:",
        "SystemRoot=C:\\Windows",
        "TEMP=C:\\Users\\anton\\AppData\\Local\\Temp",
        "TMP=C:\\Users\\anton\\AppData\\Local\\Temp",
        "USERDOMAIN=DESKTOP-8CPMMTF",
        "USERDOMAIN_ROAMINGPROFILE=DESKTOP-8CPMMTF",
        "USERNAME=anton",
        "USERPROFILE=C:\\Users\\anton",
        "VBOX_MSI_INSTALL_PATH=C:\\Program Files\\Oracle\\VirtualBox\\",
        "windir=C:\\Windows",
    ];*/
    
    let arena = Bump::new();
    let mut buffer = Vec::with_capacity(100);

    let env_vars_c_vec: bumpalo::collections::Vec<CString> = std::env::vars_os()
        .map(|(k, v)| {
            buffer.clear();

            use std::io::Write;
            buffer.write_all(os_str_as_utf8_bytes(&k)).unwrap();
            buffer.write_all(b"=").unwrap();
            buffer.write_all(os_str_as_utf8_bytes(&v)).unwrap();

            CString::new(buffer.as_slice()).unwrap()
        })
        .collect_in(&arena);

    /*let mut env_vars_c_vec: Vec<CString> = Vec::new();

    for s in env_vars.iter() {
        match CString::new(*s) {
            Ok(cs) => env_vars_c_vec.push(cs),
            Err(e) => {
                eprintln!("Error converting &str to CString: {:?}", e);
            }
        }
    }*/

    let envp: bumpalo::collections::Vec<*const c_char> = env_vars_c_vec
            .iter()
            .map(|s| s.as_ptr())
            .chain([std::ptr::null()])
            .collect_in(&arena);

    unsafe {
        libc::execve(
            prog.as_ptr(),
            argv.as_ptr(),
            envp.as_ptr(),
        );
    }
}

fn os_str_as_utf8_bytes(os_str: &OsStr) -> &[u8] {
    os_str.to_str().unwrap().as_bytes()
}
