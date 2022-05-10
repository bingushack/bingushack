use std::sync::mpsc::{Receiver, Sender};

use eframe::egui;

pub fn init_debug_console() -> (DebugConsole, Sender<String>) {
    let (tx, rx) = std::sync::mpsc::channel();
    (DebugConsole::new(rx), tx)
}

pub fn run_debug_console(app: DebugConsole) {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}

pub struct DebugConsole {
    text: Vec<String>,
    rx: Receiver<String>,
}

impl DebugConsole {
    pub fn new(rx: Receiver<String>) -> Self {
        Self {
            text: vec![String::from("start")],
            rx,
        }
    }
}

impl eframe::App for DebugConsole {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            if let Ok(text) = self.rx.try_recv() {
                self.text.push(text);
            }

            ui.hyperlink("https://github.com/bingushack/bingushack");

            ui.separator();

            ui.label(self.text.join("\n"));
        });
    }
}
