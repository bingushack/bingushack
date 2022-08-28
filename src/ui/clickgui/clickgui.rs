use std::sync::mpsc::{Receiver, Sender};
use super::{
    clickgui_message::ClickGuiMessage,
    enabled_setting::EnabledSetting,
};
use crate::client::{Client, Modules};
use crate::message_box;
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

    modules: Vec<EnabledSetting>,
}

impl ClickGui<'_> {
    pub fn new(jni_env: JNIEnv<'static>, rx: Receiver<ClickGuiMessage>) -> Self {
        let (client_sender, client_receiver) = std::sync::mpsc::channel();
        let client = Client::new(jni_env, client_receiver, client_sender.clone());
        Self {
            rx,
            client_sender,
            client,

            // prolly a better way to do this with hashmaps/hashsets in the future
            modules: vec![
                EnabledSetting::new(Modules::AutoTotem, false),
            ],
        }
    }

    pub fn get_client_sender(&self) -> Sender<ClickGuiMessage> {
        self.client_sender.clone()
    }
}

impl eframe::App for ClickGui<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            for mut enabled_setting in self.modules.iter_mut() {
                let module = enabled_setting.get_module();
                let enabled = enabled_setting.get_enabled_mut();
                ui.checkbox(enabled, format!("{:#?}", module));

                if *enabled {
                    self.client_sender.send(ClickGuiMessage::RunModule(module)).unwrap();
                }
            }


            self.client.client_tick();
        });
    }
}
