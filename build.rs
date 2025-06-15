use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    
    let lib_name = match (target_os.as_str(), target_arch.as_str()) {
        ("macos", "aarch64") => "lib/macosx_arm64/_fubon_neo.abi3.so",
        ("macos", "x86_64") => "lib/macosx_x86_64/_fubon_neo.abi3.so", 
        ("linux", "x86_64") => "lib/manylinux_x86_64/_fubon_neo.abi3.so",
        _ => {
            println!("cargo:warning=Unsupported platform: {}-{}", target_os, target_arch);
            return;
        }
    };
    
    // 檢查本地是否有 .so 文件
    if Path::new(lib_name).exists() {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("_fubon_neo.abi3.so");
        
        fs::copy(lib_name, &dest_path).expect("Failed to copy library");
        
        println!("cargo:rustc-link-search=native={}", out_dir);
        println!("cargo:rustc-link-lib=dylib=_fubon_neo.abi3");
    } else {
        println!("cargo:warning=Native library not found for platform {}-{}", target_os, target_arch);
        println!("cargo:warning=Please download the appropriate library from the project repository");
    }
}