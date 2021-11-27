use std::env;

fn main () {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("windows") => {}
        _tos => panic!("not supported OS")  
    }

    let build_type = env::var("PROFILE");
    match build_type.as_ref().map(|x| &**x) {
        Ok("debug") => { 
            println!("cargo:rustc-link-lib=vga-terminald");
            println!("cargo:rustc-link-search=native=lib/x64-Debug/lib");
         }
        Ok("release") => {
            println!("cargo:rustc-link-lib=vga-terminal");
            println!("cargo:rustc-link-search=native=lib/x64-Release/lib");
        }
        _tos => panic!("not supported profile")
    }

    
    
}