use sysinfo::{ProcessExt, System, SystemExt};

#[cfg(any(target_os = "linux", target_os = "windows"))]
#[no_mangle]
pub extern "C" fn check_svt_process() -> i32 {
    let s = System::new_all();
    let mut check_res = 1;
    for process in s.processes_by_name("qemu-ga") {
        println!("{} {}", process.pid(), process.name());
        check_res = 0;
        break;
    }
    check_res
}
