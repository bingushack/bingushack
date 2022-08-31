use std::sync::mpsc::{Receiver, Sender};

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
    unsafe { ENABLED = true; }
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "bingushack debug",
        options,
        Box::new(|_cc| Box::new(app)),
    );
    unsafe { ENABLED = false; }
}

pub struct DebugConsole {
    text: Vec<String>,
    rx: Receiver<String>,
    tx: Sender<String>,

    test1: f64,
    test2: f64,
}

impl DebugConsole {
    pub fn new(rx: Receiver<String>, tx: Sender<String>,) -> Self {
        Self {
            text: vec![String::from("")],
            rx,
            tx,

            test1: 2.0,
            test2: 10.0,
        }
    }
}

impl eframe::App for DebugConsole {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            if let Ok(text) = self.rx.try_recv() {
                self.text.insert(0, text);
            }

            ui.add(egui::Slider::new(
                &mut self.test1,
                Some(&mut self.test2),
                0.0..=100.0,
            ));

            ui.hyperlink("http://bingushack.cc");

            ui.separator();

            ui.label(self.text.join("\n"));
        });
    }
}
