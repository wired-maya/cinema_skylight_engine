use std::{thread, time::Duration, sync::{Arc, Mutex}};
use crate::window_utils::GlWindow;

pub fn main_thread(gl_window: Arc<Mutex<GlWindow>>) {
    while !gl_window.lock().unwrap().window.should_close() {
        // Results in 60fps, temporary so that computer doesnt slow down when testing
        thread::sleep(Duration::from_millis(16));
    }
}