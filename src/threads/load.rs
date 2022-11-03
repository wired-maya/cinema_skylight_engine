use std::{thread, sync::mpsc::{Receiver, Sender}};
use crate::widgets::Widget;

pub struct LoadingThread {
    pub thread: Option<thread::JoinHandle<()>>
}

impl LoadingThread {
    pub fn new(loading_receiver: Receiver<Widget>, ui_sender: Sender<Widget>) -> LoadingThread {
        let thread = thread::spawn(move || loop {
            match loading_receiver.recv() {
                Ok(mut widget) => {
                    // TODO: make this async to improve performance
                    widget.load_assets();
                    ui_sender.send(widget).unwrap();
                },
                Err(_) => {
                    println!("THREAD::LOAD::DISCONNECTED: Shutting down");
                    break;
                },
            }
        });

        LoadingThread { thread: Some(thread) }
    }
}