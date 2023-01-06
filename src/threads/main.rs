use std::{thread, sync::{mpsc::{Receiver, Sender, TryRecvError}}};

use glfw::{WindowEvent, Key, Action};

use crate::{window_utils::{GlWindow, WindowConfig}};
use super::ProgramEvent;

pub struct MainThread {
    pub thread: Option<thread::JoinHandle<()>>
}

impl MainThread {
    pub fn new(
        window_config: WindowConfig,
        main_receiver: Receiver<ProgramEvent>,
        program_sender: Sender<MainThreadEvent>
    ) -> MainThread {
        let thread = thread::spawn(move || {
            // Init window and its GL context
            let mut gl_window = GlWindow::new(window_config);
            // gl_safe::init(&mut gl_window.window);

            // Allows waiting for complete initialization
            program_sender.send(MainThreadEvent::InitComplete).unwrap();

            // Main loop where the main thread processes inputs when they come
            while !gl_window.window.should_close() {
                // Polls events from main thread
                let engine_event = match main_receiver.try_recv() {
                    Ok(event) => Some(event),
                    Err(TryRecvError::Empty) => None,
                    Err(TryRecvError::Disconnected) => {
                        println!("THREAD::MAIN::DISCONNECTED: Shutting down");
                        gl_window.window.set_should_close(true);
                        continue;
                    }
                };

                if let Some(event) = engine_event {
                    println!("ProgramEvent::{:#?}", event);
                }

                // Blocks until input is received then processes it
                gl_window.glfw.wait_events();
                for (_, event) in glfw::flush_messages(&gl_window.events) {
                    match event {
                        WindowEvent::Key(Key::Escape, _, Action::Press, _) => gl_window.window.set_should_close(true),
                        WindowEvent::Key(Key::Space, _, Action::Press, _) => program_sender.send(MainThreadEvent::AdvanceText).unwrap(),
                        _ => ()
                    }
                }
            }
        });

        MainThread { thread: Some(thread) }
    }
}

#[derive(Debug, PartialEq)]
pub enum MainThreadEvent {
    AdvanceText,
    InitComplete
}