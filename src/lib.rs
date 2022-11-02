use std::sync::mpsc::{self, Sender, Receiver};
use threads::MainThreadEvent;

pub mod widgets;
pub mod threads;
pub mod window_utils;
pub mod gl_safe;
pub mod debug;

pub use window_utils::WindowConfig;

// TODO: create global state struct that then has methods that communicate with the threads
// TODO: have a wait method that waits for all threads to complete that needs to be ran at the bottom of game

pub struct CinemaSkylightEngine {
    main_thread: MainThread
}

impl CinemaSkylightEngine {
    pub fn init(window_config: WindowConfig) -> CinemaSkylightEngine {
        // Create senders and recievers for main thread then start it
        // Main thread initializes everything then enters a loop that handles events sent to the thread,
        // which includes inputs handled by the input thread while loading.
        // Spawns input thread to give it the window's receiver.
        let (main_sender, main_receiver): (Sender<MainThreadEvent>, Receiver<MainThreadEvent>) = mpsc::channel();
        let main_thread = threads::MainThread::new(main_receiver, main_sender.clone(), window_config);

        CinemaSkylightEngine { main_thread }
    }
}