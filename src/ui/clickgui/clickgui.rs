use super::{clickgui_message::{ClickGuiMessage}};
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
    sync::{Mutex, Exclusive},
    sync::mpsc::{Receiver, Sender},
};

use eframe::egui;

// mutable statics because i am lazy and it works
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
    eframe::run_native("bingushack", options, Box::new(|_cc| Box::new(app)));  // will block on this until the window is closed
    // now it is closed and ENABLED is false
    unsafe {
        ENABLED = false;
    }
}

pub struct ClickGui {
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
        // some macros to make things easier
        //
        // this macro will make a vector containing all the modules it is given and returns it
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
            modules: {
                // in debug mode it needs to be mutable to add the TestModule but otherwise it doesn't need to be
                #[cfg(build = "debug")]
                let mut modules;
                #[cfg(not(build = "debug"))]
                let modules;

                // add all non-debug modules
                modules = modules_maker![
                    AutoTotem::new_boxed(),
                    Triggerbot::new_boxed(),
                    Esp::new_boxed()
                ];

                // if in debug add debug modules
                #[cfg(build = "debug")]
                modules.extend_from_slice(&modules_maker![
                    TestModule::new_boxed()
                ]);

                modules
            }
        }
    }
}

impl eframe::App for ClickGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            for (i, module) in self.modules.iter().enumerate() {
                // need to push ids because it was reusing ids otherwise and breaking stuff
                ui.push_id(i, |ui| {
                    ui.add(module_widget(&module.borrow()));
                });

                // shit code idc
                if let Ok(clickgui_message) = self.rx.try_recv() {
                    match clickgui_message {
                        ClickGuiMessage::RunRenderEvent => {
                            // run the render event
                            module.borrow().render_event();
                        },
                        _ => {}
                    }
                }

                // if module is enabled, send a message to the Client to tick the module pointed to by the message
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
        self.client.lock().unwrap().client_tick();  // maybe make client_tick take a vec of things to tick instead of queuing messages? locks the Client to do all the ticks for each module at once
        ctx.request_repaint();  // repaint it because otherwise it wouldn't work i forget why this is needed but it is
    }
}
