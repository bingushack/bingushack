use std::sync::mpsc::{Receiver, Sender};
use super::{
    clickgui_message::ClickGuiMessage,
};
use crate::client::Client;
use jni::JNIEnv;
use crate::ui::widgets::module_widget;

use eframe::egui;


static mut ENABLED: bool = false;



pub fn init_clickgui(jni_env: JNIEnv<'static>) -> (ClickGui, Sender<ClickGuiMessage>) {
    let (ntx, nrx) = std::sync::mpsc::channel();
    (ClickGui::new(jni_env, nrx), ntx)
}

pub fn run_clickgui(app: ClickGui) {
    if unsafe { ENABLED } {
        return;
    }
    // else
    unsafe { ENABLED = true; }
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "bingushack",
        options,
        Box::new(|_cc| Box::new(app)),
    );
    unsafe { ENABLED = false; }
}

pub struct ClickGui {
    rx: Receiver<ClickGuiMessage>,

    // sender to the client itself
    client_sender: Sender<ClickGuiMessage>,
    client: Client,
}

impl ClickGui {
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

impl eframe::App for ClickGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            /*
            for mut module in self.modules.iter_mut() {

                ui.add(module_widget(&mut module));

                if module.get_enabled() {
                    self.client_sender.send(ClickGuiMessage::RunModule(module.clone())).unwrap();
                }
            }


            self.client.client_tick();
            */
        });
    }
}
