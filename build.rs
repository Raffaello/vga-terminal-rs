use std::env;
use std::path::Path;
use std::path::PathBuf;
//use std::fs::copy;

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    return PathBuf::from(path);
}

fn main () {
    println!("cargo:warning=CWD is {:?}", env::current_dir().unwrap());
    println!("cargo:warning=OUT_DIR is {:?}", env::var("OUT_DIR").unwrap());
    println!("cargo:warning=CARGO_MANIFEST_DIR is {:?}", env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:warning=PROFILE is {:?}", env::var("PROFILE").unwrap());

    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("windows") => {}
        _tos => panic!("not supported OS")  
    }

    let od = get_output_path();
    println!("cargo:warning=OUT_DIR is {:?}", od);

    let build_type = env::var("PROFILE");
    // unable to copy the DLL in the OUT_DIR, so need to be copied manually.
    match build_type.as_ref().map(|x| &**x) {
        Ok("debug") => { 
            println!("cargo:rustc-link-lib=vga-terminald");
            println!("cargo:rustc-link-search=native=lib/x64-Debug/lib");
            //let files = ["vga-terminald.dll", "SDL2.dll"];
            //files.map(|f| copy(format!("{}{}", "lib/x64-Debug/lib/", f), &od).unwrap());
            //copy("lib/x64-Debug/lib/vga-terminald.dll", &od).unwrap();
         }
        Ok("release") => {
            println!("cargo:rustc-link-lib=vga-terminal");
            println!("cargo:rustc-link-search=native=lib/x64-Release/lib");
        }
        _tos => panic!("not supported profile")
    }

    

    
    
}