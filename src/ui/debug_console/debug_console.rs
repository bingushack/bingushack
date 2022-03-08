extern crate native_windows_derive as nwd;

use std::sync::mpsc::{Sender, Receiver, RecvError};
use crate::ui::debug_console::message::Message;

use nwd::NwgUi;
use nwg::{NativeUi, Window, Button, stop_thread_dispatch, dispatch_thread_events, dispatch_thread_events_with_callback};


#[derive(Default, NwgUi)]
pub struct DebugConsole {
    #[nwg_control(size: (300, 125), position: (100, 100), title: "bingushack debug", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [DebugConsole::close] )]
    window: Window,

    #[nwg_control(text: "close", size: (100, 45), position: (10, 40))]
    #[nwg_events( OnButtonClick: [BasicApp::close] )]
    close_button: Button
}

impl DebugConsole {
    pub fn close(&self) {
        stop_thread_dispatch();
    }
}

pub fn run_debug_console(rx: Receiver<Message>) -> u32 {
    nwg::init().unwrap();

    let debug_console = DebugConsole::default();

    dispatch_thread_events();

    dispatch_thread_events_with_callback(|| {
        match rx.recv() {
            Ok(message) => match message {
                Message::KillGUI => debug_console.close(),
                _ => {},
            }
            Err(_) => {}
        }
    });

    0
}
