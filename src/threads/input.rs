use std::{thread, sync::mpsc::{Receiver, Sender}};
use glfw::{WindowEvent, Key, Action};
use super::main::MainThreadEvent;

pub struct InputThread {
    pub thread: thread::JoinHandle<()>
}

impl InputThread {
    pub fn new(events: Receiver<(f64, WindowEvent)>, main_sender: Sender<MainThreadEvent>) -> InputThread {
        let thread = thread::spawn(move || loop {
            // Recv is blocking so this doesn't run unless it's needed
            match events.recv() {
                Ok((_, event)) => {
                    match event {
                        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                            main_sender.send(MainThreadEvent::CloseWindow).unwrap();
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