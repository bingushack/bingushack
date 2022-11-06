use super::{clickgui_message::{ClickGuiMessage}};
use crate::{
    client::{
        module::{modules::*, BingusModule},
        BoxedBingusModule, Client,
    },
    ui::widgets::module_widget, OLD_CONTEXT, NEW_CONTEXT, log_to_file, STATIC_HDC,
};
use glutin::platform::windows::HGLRC;
use jni::JNIEnv;
use once_cell::sync::OnceCell;
use winapi::{um::wingdi::{wglGetCurrentContext, wglMakeCurrent, wglGetCurrentDC, wglCreateContext}, shared::windef::{HGLRC__, HDC, HDC__}};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Mutex, atomic::AtomicPtr},
    sync::mpsc::{Receiver, Sender},
};

use eframe::egui;

// mutable statics because i am lazy and it works
static mut ENABLED: bool = false;

// will need one for debug console too if this works
static mut CLICKGUI_CONTEXT: OnceCell<HGLRC> = OnceCell::new();
static mut CLICKGUI_HDC: OnceCell<HDC> = OnceCell::new();

pub fn init_clickgui(jni_env: JNIEnv<'static>) -> (ClickGui, Sender<ClickGuiMessage>, Receiver<()>) {
    let (ntx, nrx) = std::sync::mpsc::channel();
    let (ctx, crx) = std::sync::mpsc::channel();
    (ClickGui::new(jni_env, nrx, ctx), ntx, crx)
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
    send_messages_back_sender: Sender<()>,
    client: Mutex<Client>, // why does the ClickGui contain the Client and not the other way around????
    // why are the modules in the ClickGui wtf???
    // prolly a better way to do this with hashmaps/hashsets in the future
    modules: Vec<Rc<RefCell<BoxedBingusModule>>>,
}

impl ClickGui {
    pub fn new(jni_env: JNIEnv<'static>, rx: Receiver<ClickGuiMessage>, send_messages_back_sender: Sender<()>) -> Self {
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
            send_messages_back_sender,
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
        let _ = unsafe {
            CLICKGUI_HDC.get_or_init(|| wglGetCurrentDC())
        };
        let _ = unsafe {
            CLICKGUI_CONTEXT.get_or_init(|| wglGetCurrentContext())
        };

        let mut do_render_event = false;
        // shit code idc
        if let Ok(clickgui_message) = self.rx.try_recv() {
            match clickgui_message {
                ClickGuiMessage::RunRenderEvent => {
                    do_render_event = true;
                },
                _ => {}
            }
        }

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            for (i, module) in self.modules.iter().enumerate() {
                // need to push ids because it was reusing ids otherwise and breaking stuff
                ui.push_id(i, |ui| {
                    ui.add(module_widget(&module.borrow()));
                });

                if do_render_event {
                    module.borrow().render_event();
                    // use send_messages_back_sender to send a message to stop waiting
                    let _ = self.send_messages_back_sender.send(()).unwrap();
                }

                // if module is enabled, send a message to the Client to tick the module pointed to by the message
                let module_enabled = module.borrow().get_enabled();
                if module_enabled {
                    self.client_sender
                        .send(ClickGuiMessage::RunModule(Rc::clone(module)))
                        .unwrap();
                }
            }
        });
        self.client.lock().unwrap().client_tick();  // maybe make client_tick take a vec of things to tick instead of queuing messages? locks the Client to do all the ticks for each module at once

        // set the correct context
        unsafe {
            let hdc = *CLICKGUI_HDC.get().unwrap();
            let context = CLICKGUI_CONTEXT.get_mut().unwrap();
            wglMakeCurrent(hdc, *context);
        }
        ctx.request_repaint();  // repaint it because otherwise it wouldn't work i forget why this is needed but it is. but it also breaks things. so idk.
    }
}
