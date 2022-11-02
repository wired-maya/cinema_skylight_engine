use std::{thread, sync::{mpsc::Receiver, Arc, Mutex}};
use glfw::{WindowEvent, Key, Action};

use crate::window_utils::GlWindow;

pub struct InputThread {
    pub thread: thread::JoinHandle<()>
}

impl InputThread {
    pub fn new(events: Receiver<(f64, WindowEvent)>, gl_window: Arc<Mutex<GlWindow>>) -> InputThread {
        let thread = thread::spawn(move || loop {
            // Recv is blocking so this doesn't run unless it's needed
            match events.recv() {
                Ok((_, event)) => {
                    match event {
                        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                            gl_window.lock().unwrap().window.set_should_close(true);
                        },
                        _ => ()
                    }
                },
                Err(_) => {
                    println!("THREAD::INPUT::DISCONNECTED");
                    break;
                },
            }
        });

        InputThread { thread }
    }
}