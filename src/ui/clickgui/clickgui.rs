use std::sync::mpsc::{Receiver, Sender};
use super::clickgui_message::ClickGuiMessage;
use crate::client::Client;
use jni::JNIEnv;

use eframe::egui;

pub fn init_clickgui<'c>(jni_env: JNIEnv<'static>) -> (ClickGui<'c>, Sender<ClickGuiMessage>) {
    let (ntx, nrx) = std::sync::mpsc::channel();
    (ClickGui::new(jni_env, nrx), ntx)
}

pub fn run_clickgui(app: ClickGui<'static>) {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "bingushack",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}

pub struct ClickGui<'c> {
    rx: Receiver<ClickGuiMessage>,

    // sender to the client itself
    client_sender: Sender<ClickGuiMessage>,
    client: Client<'c>,
}

impl ClickGui<'_> {
    pub fn new(jni_env: JNIEnv<'static>, rx: Receiver<ClickGuiMessage>) -> Self {
        let (client_sender, client_receiver) = std::sync::mpsc::channel();
        let client = Client::new(jni_env, client_receiver, client_sender.clone());
        Self {
            rx,
            client_sender,
            client,
        }
    }

    pub fn get_client_sender(&self) -> Sender<ClickGuiMessage> {
        self.client_sender.clone()
    }
}

impl eframe::App for ClickGui<'_> {
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

            if ui.button("do a thing").clicked() {
                self.client_sender.send(ClickGuiMessage::Dev("Hello world!".to_string())).unwrap();
                self.client.client_tick();
            }
        });
    }
}
