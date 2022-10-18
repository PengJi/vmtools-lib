use std::ffi::{OsStr, OsString};
use std::io;

#[cfg(target_os = "linux")]
use libc;
#[cfg(target_os = "linux")]
use std::os::unix::ffi::{OsStrExt, OsStringExt};

#[cfg(target_os = "windows")]
use std::os::windows::ffi::{OsStrExt, OsStringExt};
#[cfg(target_os = "windows")]
use std::ptr;
#[cfg(target_os = "windows")]
use winapi::um::sysinfoapi;

// use std::io;
// fn try_main() -> io::Result<()> {
//     let name = hostname::get_hostname()?;
//     Ok(())
// }
// fn main() {
//     try_main().unwrap();
// }
//
// use std::io;
// fn try_main() -> io::Result<()> {
//     hostname::set_hostname("potato")?;
//     Ok(())
// }
// fn main() {
//    try_main().unwrap();
// }

#[cfg(target_os = "linux")]
fn wrap_buffer(mut bytes: Vec<u8>) -> OsString {
    // Returned name might be truncated if it does not fit
    // and `buffer` will not contain the trailing \0 in that case.
    // Manually capping the buffer length here.
    let end = bytes
        .iter()
        .position(|&byte| byte == 0x00)
        .unwrap_or_else(|| bytes.len());
    bytes.resize(end, 0x00);

    OsString::from_vec(bytes)
}

#[cfg(target_os = "linux")]
pub fn get_linux_hostname() -> io::Result<OsString> {
    // According to the POSIX specification,
    // host names are limited to `HOST_NAME_MAX` bytes
    //
    // https://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html
    let size = unsafe { libc::sysconf(libc::_SC_HOST_NAME_MAX) as libc::size_t };
    let mut buffer = vec![0u8; size];
    let result = unsafe { libc::gethostname(buffer.as_mut_ptr() as *mut libc::c_char, size) };
    if result != 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(wrap_buffer(buffer))
}

#[cfg(target_os = "linux")]
pub fn set_linux_hostname(hostname: &OsStr) -> io::Result<()> {
    #[allow(non_camel_case_types)]
    type hostname_len_t = libc::size_t;

    let size = hostname.len() as hostname_len_t;
    let result =
        unsafe { libc::sethostname(hostname.as_bytes().as_ptr() as *const libc::c_char, size) };

    if result != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use super::wrap_buffer;

    // Happy path case: there is a correct null terminated C string in a buffer
    // and a bunch of NULL characters from the pre-allocated buffer
    #[test]
    fn test_non_overflowed_buffer() {
        let buf = b"potato\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".to_vec();

        assert_eq!(wrap_buffer(buf), OsStr::new("potato"));
    }

    #[test]
    fn test_empty_buffer() {
        let buf = b"".to_vec();

        assert_eq!(wrap_buffer(buf), OsStr::new(""));
    }

    #[test]
    fn test_filled_with_null_buffer() {
        let buf = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".to_vec();

        assert_eq!(wrap_buffer(buf), OsStr::new(""));
    }

    // Hostname value had overflowed the buffer, so it was truncated
    // and according to the POSIX documentation of the `gethostname`:
    //
    // > it is unspecified whether the returned name is null-terminated.
    #[test]
    fn test_overflowed_buffer() {
        let buf = b"potat".to_vec();

        assert_eq!(wrap_buffer(buf), OsStr::new("potat"));
    }
}

#[cfg(target_os = "windows")]
pub fn get_windows_hostname() -> io::Result<OsString> {
    let mut size = 0;
    unsafe {
        // Don't care much about the result here,
        // it is guaranteed to return an error,
        // since we passed the NULL pointer as a buffer
        let result = sysinfoapi::GetComputerNameExW(
            sysinfoapi::ComputerNamePhysicalDnsHostname,
            ptr::null_mut(),
            &mut size,
        );
        debug_assert_eq!(result, 0);
    };

    let mut buffer = Vec::with_capacity(size as usize);
    let result = unsafe {
        sysinfoapi::GetComputerNameExW(
            sysinfoapi::ComputerNamePhysicalDnsHostname,
            buffer.as_mut_ptr(),
            &mut size,
        )
    };

    if result == 0 {
        Err(io::Error::last_os_error())
    } else {
        unsafe {
            buffer.set_len(size as usize);
        }

        Ok(OsString::from_wide(&buffer))
    }
}

#[cfg(target_os = "windows")]
pub fn set_windows_hostname(hostname: &OsStr) -> io::Result<()> {
    let buffer = hostname.encode_wide().collect::<Vec<_>>();
    let result = unsafe {
        sysinfoapi::SetComputerNameExW(sysinfoapi::ComputerNamePhysicalDnsHostname, buffer.as_ptr())
    };

    if result == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
