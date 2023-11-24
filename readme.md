不用重启
# 调用 so
```sh
cd apis
cargo build
gcc -g main.c -o call_rust -lfoo -L./apis/target/debug/
LD_LIBRARY_PATH=/root/codestore/lang/rust/api_lib/api/target/debug ./call_rust
```

# usage for sysinfo
```rust
use sysinfo::{ProcessExt, System, SystemExt};

#[cfg(windows)] extern crate winapi;
use std::io::Error;

// #[cfg(target_os = "windows")]
#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}

#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}
```

# call rust functions from c on linux
```rust
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::slice;

use sysinfo::{ProcessExt, System, SystemExt};

// A Rust struct mapping the C struct
#[repr(C)]
#[derive(Debug)]
pub struct RustStruct {
    pub c: char,
    pub ul: u64,
    pub c_string: *const c_char,
}

macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident, $ctype:ty) => {
        #[no_mangle]
        pub extern "C" fn $func_name(v: $ctype) {
            // The `stringify!` macro converts an `ident` into a string.
            println!(
                "{:?}() is called, value passed = <{:?}>",
                stringify!($func_name),
                v
            );
        }
    };
}

// create simple functions where C type is exactly mapping a Rust type
create_function!(rust_char, char);
create_function!(rust_wchar, char);
create_function!(rust_short, i16);
create_function!(rust_ushort, u16);
create_function!(rust_int, i32);
create_function!(rust_uint, u32);
create_function!(rust_long, i64);
create_function!(rust_ulong, u64);
create_function!(rust_void, *mut c_void);

// for NULL-terminated C strings, it's a little bit clumsier
#[no_mangle]
pub extern "C" fn rust_string(c_string: *const c_char) {
    // build a Rust string from C string
    let s = unsafe { CStr::from_ptr(c_string).to_string_lossy().into_owned() };

    println!("rust_string() is called, value passed = <{:?}>", s);
}

// for C arrays, need to pass array size
#[no_mangle]
pub extern "C" fn rust_int_array(c_array: *const i32, length: usize) {
    // build a Rust array from array & length
    let rust_array: &[i32] = unsafe { slice::from_raw_parts(c_array, length as usize) };
    println!(
        "rust_int_array() is called, value passed = <{:?}>",
        rust_array
    );
}

#[no_mangle]
pub extern "C" fn rust_string_array(c_array: *const *const c_char, length: usize) {
    // build a Rust array from array & length
    let tmp_array: &[*const c_char] = unsafe { slice::from_raw_parts(c_array, length as usize) };

    // convert each element to a Rust string
    let rust_array: Vec<_> = tmp_array
        .iter()
        .map(|&v| unsafe { CStr::from_ptr(v).to_string_lossy().into_owned() })
        .collect();

    println!(
        "rust_string_array() is called, value passed = <{:?}>",
        rust_array
    );
}

// for C structs, need to convert each individual Rust member if necessary
#[no_mangle]
pub unsafe extern "C" fn rust_cstruct(c_struct: *mut RustStruct) {
    let rust_struct = &*c_struct;
    let s = CStr::from_ptr(rust_struct.c_string).to_string_lossy().into_owned();

    println!(
        "rust_cstruct() is called, values passed = <{} {} {}>",
        rust_struct.c, rust_struct.ul, s
    );
}

#[no_mangle]
pub extern "C" fn check_svt_process() -> i32 {
    let s = System::new_all();
    let mut check_res = 1;
    for process in s.processes_by_name("qemu-ga") {
        // println!("{} {}", process.pid(), process.name());
        check_res = 0;
    }
    check_res
}
```

# 参考
[How to call Rust functions from C on Linux](https://dev.to/dandyvica/how-to-call-rust-functions-from-c-on-linux-h37)  
[Cross-compilation in Rust](https://kerkour.com/rust-cross-compilation)  
[Foreign Function Interface](https://learnku.com/docs/nomicon/2018/110-ffi/4756#662575)  
