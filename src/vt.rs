pub mod vt {
    use std::{ffi::CStr, os::raw::c_char};

    //#[link(name = "vga-terminald")]
    extern {
        fn VGA_TERMINAL_version() -> *const c_char;
    }
    
    pub fn vt_version() -> String {
        unsafe {
            CStr::from_ptr(VGA_TERMINAL_version()).to_string_lossy().into_owned()
        }
    }
}