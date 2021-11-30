pub mod vt {
    use std::{ffi::{CStr, c_void}, os::raw::c_char};

    pub type VGATerminal = *mut c_void;

    //#[link(name = "vga-terminal")]
    extern "C" {
        fn VGA_TERMINAL_version() -> *const c_char;
        fn VGA_TERMINAL_init() -> VGATerminal;
        fn VGA_TERMINAL_destroy(term: VGATerminal);
    }
    
    pub fn vt_version() -> String {
        unsafe {
            CStr::from_ptr(VGA_TERMINAL_version()).to_string_lossy().into_owned()
        }
    }

    pub fn vt_init() -> VGATerminal {
        let v = unsafe { VGA_TERMINAL_init() };
        return v;
    }

    pub fn vt_destroy(term: VGATerminal) {
        unsafe { VGA_TERMINAL_destroy(term) }
    }
}