#[cfg(test)]
mod tests {
    use std::{ptr};

    use crate::vt::vt::{vt_destroy, vt_init, vt_version};

    #[test]
    fn test_vt_version() {
        let version = vt_version();
        const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
        assert_eq!(version, PKG_VERSION);
    }

    #[test]
    fn test_vt_init_fail() {
        let term = vt_init();
        assert_eq!(term, ptr::null_mut());
    }
    
    #[test]
    fn test_vt_init() {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let timer_subsystem = sdl_context.timer().unwrap();
        let wb = video_subsystem.window("title", 320, 200);
        let w = wb.build().unwrap();
        //video_subsystem.
        //std::thread::sleep(std::time::Duration::from_millis(1000));
        let term = vt_init();
        assert_ne!(term, ptr::null_mut());
        vt_destroy(term);
    }

    #[test]
    fn test_vt_destroy() {
        let term = vt_init();
        vt_destroy(term);
        assert_eq!(true, true);
    }
}
