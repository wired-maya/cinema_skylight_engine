use std::sync::mpsc::{self, Sender, Receiver};
use threads::{ProgramEvent, MainThreadEvent};

pub mod widgets;
pub mod threads;
pub mod window_utils;
pub mod gl_safe;
pub mod debug;

pub use window_utils::WindowConfig;

// TODO: create global state struct that then has methods that communicate with the threads
// TODO: have a wait method that waits for all threads to complete that needs to be ran at the bottom of game

pub struct CinemaSkylightEngine {
    program_receiver: Receiver<MainThreadEvent>,
    main_thread: threads::MainThread,
    main_sender: Option<Sender<ProgramEvent>>,
}

impl CinemaSkylightEngine {
    pub fn init(window_config: WindowConfig) -> CinemaSkylightEngine {
        let (program_sender, program_receiver) = mpsc::channel();

        // Create main thread that delegates work to other threads. Receives events from program thread
        // and handles the rest.
        let (main_sender, main_receiver) = mpsc::channel();
        let main_thread = threads::MainThread::new(window_config, main_receiver, program_sender);

        let engine = CinemaSkylightEngine { program_receiver, main_thread, main_sender: Some(main_sender) };

        // Waits for initialization to be complete before continuing
        engine.wait_for_advance();

        engine
    }

    pub fn wait_for_advance(&self) {
        // Block until advance signal is given
        let event = self.program_receiver.recv().unwrap();
        println!("MainThreadEvent::{:#?}", event);
    }

    // Satiates the main thread for debug purposes, preventing the window from becoming non-responsive
    pub fn satiate_main_thread(&self) {
        self.main_sender.as_ref().unwrap().send(ProgramEvent::Debug).unwrap();
    }
}

impl Drop for CinemaSkylightEngine {
    fn drop(&mut self) {
        // Gracefully exit all threads
        drop(self.main_sender.take());

        println!("Shutting down threads");

        if let Some(thread) = self.main_thread.thread.take() {
            thread.join().unwrap();
        }
    }
}