use core::time;

use vga_terminal_rs::vt::vt::{vt_destroy, vt_init};

use std::{thread};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let term = vt_init();
    
    
    let delay = time::Duration::from_millis(1000);
    thread::sleep(delay);

    vt_destroy(term);
}