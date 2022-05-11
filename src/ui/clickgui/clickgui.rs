use std::sync::mpsc::{Receiver, Sender};
use super::clickgui_message::ClickGuiMessage;

use eframe::egui;

pub fn init_clickgui(tx: Sender<ClickGuiMessage>) -> (ClickGui, Sender<ClickGuiMessage>) {
    let (ntx, nrx) = std::sync::mpsc::channel();
    (ClickGui::new(nrx, tx), ntx)
}

pub fn run_clickgui(app: ClickGui) {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "bingushack",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}

pub struct ClickGui {
    rx: Receiver<ClickGuiMessage>,
    tx: Sender<ClickGuiMessage>,
}

impl ClickGui {
    pub fn new(rx: Receiver<ClickGuiMessage>, tx: Sender<ClickGuiMessage>) -> Self {
        Self {
            rx,
            tx,
        }
    }
}

impl eframe::App for ClickGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            if let Ok(message) = self.rx.try_recv() {
                match message {
                    ClickGuiMessage::StringMessage(text) => {
                        ui.label(text);
                    }
                    _ => {}
                }
            }
        });
    }
}
