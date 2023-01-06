use std::sync::mpsc::{self, Sender, Receiver};
use threads::{ProgramEvent, MainThreadEvent};

pub mod widgets;
pub mod threads;
pub mod window_utils;

use widgets::*;
pub use window_utils::*;

pub struct CinemaSkylightEngine {
    program_receiver: Receiver<MainThreadEvent>,
    main_thread: threads::MainThread,
    main_sender: Option<Sender<ProgramEvent>>,
    loading_thread: threads::LoadingThread,
    loading_sender: Option<Sender<Widget>>,
    ui_thread: threads::UiThread,
    ui_sender: Option<Sender<Widget>>
}

impl CinemaSkylightEngine {
    pub fn init(window_config: WindowConfig) -> CinemaSkylightEngine {
        let (program_sender, program_receiver) = mpsc::channel();

        // Channels for communication between the threads
        let (main_sender, main_receiver) = mpsc::channel();
        let (loading_sender, loading_receiver) = mpsc::channel();
        let (ui_sender, ui_receiver) = mpsc::channel();

        // Create main thread that delegates work to other threads. Receives events from program thread
        // and handles the rest.
        let main_thread = threads::MainThread::new(window_config, main_receiver, program_sender.clone());
        // Create a thread that handles loading all assets sent to it by the main thread
        let loading_thread = threads::LoadingThread::new(loading_receiver, ui_sender.clone());
        // Create UI thread that loops every frame
        let ui_thread = threads::UiThread::new(ui_receiver, program_sender);

        CinemaSkylightEngine {
            program_receiver,
            main_thread,
            main_sender: Some(main_sender),
            loading_thread,
            loading_sender: Some(loading_sender),
            ui_thread,
            ui_sender: Some(ui_sender)
        }
    }

    pub fn wait_for_advance(&self) {
        loop {
            // Block until advance signal is given
            let event = self.program_receiver.recv().unwrap();
            println!("MainThreadEvent::{:#?}", event);

            if event == MainThreadEvent::AdvanceText {
                break; // Only break if advance signal is given
            }
        }
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
        drop(self.loading_sender.take());
        drop(self.ui_sender.take());

        println!("Shutting down threads");

        if let Some(thread) = self.main_thread.thread.take() {
            thread.join().unwrap();
        }

        if let Some(thread) = self.loading_thread.thread.take() {
            thread.join().unwrap();
        }

        if let Some(thread) = self.ui_thread.thread.take() {
            thread.join().unwrap();
        }
    }
}