use super::clickgui_message::ClickGuiMessage;
use crate::{
    client::{
        module::{modules::*, BingusModule},
        BoxedBingusModule, Client,
    },
    ui::widgets::module_widget,
};
use jni::JNIEnv;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::Mutex,
    sync::mpsc::{Receiver, Sender},
};

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
    unsafe {
        ENABLED = true;
    }
    let options = eframe::NativeOptions::default();
    eframe::run_native("bingushack", options, Box::new(|_cc| Box::new(app)));
    unsafe {
        ENABLED = false;
    }
}

pub struct ClickGui {
    #[allow(dead_code)]
    rx: Receiver<ClickGuiMessage>,

    // sender to the client itself
    client_sender: Sender<ClickGuiMessage>,
    client: Mutex<Client>, // why does the ClickGui contain the Client and not the other way around????
    // why are the modules in the ClickGui wtf???
    // prolly a better way to do this with hashmaps/hashsets in the future
    modules: Vec<Rc<RefCell<BoxedBingusModule>>>,
}

impl ClickGui {
    pub fn new(jni_env: JNIEnv<'static>, rx: Receiver<ClickGuiMessage>) -> Self {
        let (client_sender, client_receiver) = std::sync::mpsc::channel();
        let client = Mutex::new(Client::new(jni_env, client_receiver, client_sender.clone()));
        macro_rules! modules_maker {
            ($($module:expr),*) => {{
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push(Rc::new(RefCell::new($module)));
                )*
                temp_vec
            }}
        }
        Self {
            rx,
            client_sender,
            client,
            modules: modules_maker!{
                AutoTotem::new_boxed(),
                Triggerbot::new_boxed(),

                #[cfg(build = "debug")]
                TestModule::new_boxed()
            },
        }
    }
}

impl eframe::App for ClickGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            for (i, module) in self.modules.iter().enumerate() {
                ui.push_id(i, |ui| {
                    ui.add(module_widget(&module.borrow()));
                });

                if module
                    .borrow()
                    .get_enabled_setting()
                    .lock()
                    .unwrap()
                    .borrow()
                    .get_value()
                    .try_into()
                    .unwrap()
                {
                    self.client_sender
                        .send(ClickGuiMessage::RunModule(Rc::clone(module)))
                        .unwrap();
                }
            }
        });
        self.client.lock().unwrap().client_tick();
        ctx.request_repaint();
    }
}
