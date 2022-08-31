use std::sync::mpsc::{Receiver, Sender};
use super::{
    clickgui_message::ClickGuiMessage,
};
use crate::client::{Client, BoxedBingusModule};
use jni::JNIEnv;
use crate::ui::widgets::module_widget;
use crate::client::module::{
    BingusModule,
    modules::AutoTotem
};
use std::rc::Rc;
use std::cell::RefCell;

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
    client: Client,  // why does the ClickGui contain the Client and not the other way around????
    // why are the modules in the ClickGui wtf???
    // prolly a better way to do this with hashmaps/hashsets in the future
    modules: Vec<Rc<RefCell<BoxedBingusModule>>>,
}

impl ClickGui {
    pub fn new(jni_env: JNIEnv<'static>, rx: Receiver<ClickGuiMessage>) -> Self {
        let (client_sender, client_receiver) = std::sync::mpsc::channel();
        let client = Client::new(jni_env, client_receiver, client_sender.clone());
        Self {
            rx,
            client_sender,
            client,
            modules: vec![
                Rc::new(RefCell::new(AutoTotem::new_boxed())),
            ],
        }
    }

    pub fn get_client_sender(&self) -> Sender<ClickGuiMessage> {
        self.client_sender.clone()
    }
}

impl eframe::App for ClickGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {

            for module in self.modules.iter() {

                ui.add(module_widget(&module.borrow()));

                if module.borrow().get_enabled_ref_cell().borrow().get_value().try_into().unwrap() {
                    self.client_sender.send(ClickGuiMessage::RunModule(
                        Rc::clone(module)
                    )).unwrap();
                }
            }


            self.client.client_tick();
        });
    }
}
