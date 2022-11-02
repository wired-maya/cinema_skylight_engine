use std::{thread, sync::{mpsc::{Receiver, Sender}}};

use crate::{window_utils::{GlWindow, WindowConfig}, gl_safe};
use super::input;

pub struct MainThread {
    pub thread: thread::JoinHandle<()>
}

impl MainThread {
    pub fn new(
        receiver: Receiver<MainThreadEvent>,
        main_sender: Sender<MainThreadEvent>,
        window_config: WindowConfig
    ) -> MainThread {
        let thread = thread::spawn(move || {
            // Init window and its GL context
            let mut gl_window = GlWindow::new(window_config);
            gl_safe::init(&mut gl_window.window);

            // Move events reciever out and into the input thread (unwrap never fails here)
            let events = gl_window.events.take().unwrap();
            let _input_thread = input::InputThread::new(events, main_sender);

            // Main loop where the main thread processes inputs when they come
            while !gl_window.window.should_close() {
                // Recv is blocking so this doesn't run unless it's needed
                match receiver.recv() {
                    Ok(MainThreadEvent::CloseWindow) => gl_window.window.set_should_close(true),
                    Err(_) => {
                        println!("THREAD::MAIN::DISCONNECTED");
                        break;
                    }
                }
            }
        });

        MainThread { thread }
    }
}

pub enum MainThreadEvent {
    CloseWindow
}