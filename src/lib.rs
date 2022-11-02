use std::sync::{Mutex, Arc};

use window_utils::GlWindow;

pub mod widgets;
pub mod threads;
pub mod window_utils;
pub mod gl_safe;
pub mod debug;

// TODO: create global state struct that then has methods that communicate with the threads

pub fn init(width: u32, height: u32, title: &str) {
    // Init window and its GL context
    let mut gl_window = GlWindow::new(width, height, title);
    gl_safe::init(&mut gl_window.window);

    // Move events reciever out and into the input thread (unwrap never fails here)
    let events = gl_window.events.take().unwrap();
    let _input_thread = threads::InputThread::new(events);

}