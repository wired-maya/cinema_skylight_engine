use std::{thread, sync::mpsc::{Receiver, Sender, TryRecvError}, ops::{Deref, DerefMut}};
use crate::widgets::{Widget, Drawable};

use super::MainThreadEvent;

pub struct UiThread {
    pub thread: Option<thread::JoinHandle<()>>
}

impl UiThread {
    pub fn new(ui_receiver: Receiver<Widget>, program_sender: Sender<MainThreadEvent>) -> UiThread {
        let thread = thread::spawn(move || {
            // TODO: find a way to have this accessible outside the thread
            let mut render_stack = RenderStack::new();

            loop {
                match ui_receiver.try_recv() {
                    Ok(widget) => render_stack.push(widget),
                    Err(TryRecvError::Empty) => (),
                    Err(TryRecvError::Disconnected) => {
                        println!("THREAD::UI::DISCONNECTED: Shutting down");
                        break;
                    }
                }

                // TODO: Surround with general OpenGL context calls, etc.
                render_stack.draw_all();
            }
        });

        UiThread { thread: Some(thread) }
    }
}

pub struct RenderStack(Vec<Widget>);

impl RenderStack {
    pub fn new() -> RenderStack {
        RenderStack(Vec::with_capacity(32)) // Allows 32 widgets before re-allocating
    }

    pub fn draw_all(&self) {
        for widget in self.0.iter() {
            widget.draw();
        }
    }
}

impl Deref for RenderStack {
    type Target = Vec<Widget>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RenderStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}