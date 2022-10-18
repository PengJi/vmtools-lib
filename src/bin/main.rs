use apis::svt_process;
use apis::hostname;

fn main() {
    let check_res = svt_process::check_svt_process();
    print!("check svt process result: {}\n", check_res);

    let hostname_str = hostname::get_linux_hostname();
    match hostname_str {
        Ok(host_name) => println!("hostname: {}", host_name.to_string_lossy()),
        Err(e) => println!("Error: {}", e),
    }
}
