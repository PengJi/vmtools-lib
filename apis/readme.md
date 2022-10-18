# 安装 cross
`cargo install -f cross`

## compiling for linux
```sh
cross build --target x86_64-unknown-linux-gnu
```

## Cross-compiling to aarch64 (arm64)
```sh
cross build --target aarch64-unknown-linux-gnu
```

## Cross-compiling from Linux to Windows
```sh
cross build --target x86_64-pc-windows-gnu
```

# 参考
[GuillaumeGomez/sysinfo](https://github.com/GuillaumeGomez/sysinfo)  
[svartalf/hostname](https://github.com/svartalf/hostname)
[retep998/winapi-rs](https://github.com/retep998/winapi-rs)  
[Windows APIs](https://lib.rs/os/windows-apis)  