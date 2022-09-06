use std::sync::mpsc::{Receiver, Sender};
use super::DoubleSlider;

use eframe::egui;

static mut ENABLED: bool = false;

pub fn init_debug_console() -> (DebugConsole, Sender<String>) {
    let (tx, rx) = std::sync::mpsc::channel();
    (DebugConsole::new(rx, tx.clone()), tx)
}

pub fn run_debug_console(app: DebugConsole) {
    if unsafe { ENABLED } {
        return;
    }
    // else
    unsafe {
        ENABLED = true;
    }
    let options = eframe::NativeOptions::default();
    eframe::run_native("bingushack debug", options, Box::new(|_cc| Box::new(app)));
    unsafe {
        ENABLED = false;
    }
}

pub struct DebugConsole {
    double_slider_values: [f64; 2],
    text: Vec<String>,
    rx: Receiver<String>,
    #[allow(dead_code)]
    tx: Sender<String>,
}

impl DebugConsole {
    pub fn new(rx: Receiver<String>, tx: Sender<String>) -> Self {
        Self {
            double_slider_values: [3.0, 7.0],
            text: vec![String::from("")],
            rx,
            tx,
        }
    }
}

impl eframe::App for DebugConsole {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            if let Ok(text) = self.rx.try_recv() {
                self.text.insert(0, text);
            }

            ui.hyperlink("http://bingushack.cc");

            ui.separator();

            ui.add(DoubleSlider::new(&mut self.double_slider_values, 0.0..=20.0));

            ui.label(self.text.join("\n"));
        });
    }
}
