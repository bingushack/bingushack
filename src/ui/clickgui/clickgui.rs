use crate::{
    client::{
        module::{modules::*, BingusModule},
        BoxedBingusModule,
    },
    ui::widgets::module_widget, OLD_CONTEXT, NEW_CONTEXT, log_to_file, STATIC_HDC, RENDER_MANAGER, RenderManager, managers::ModulesRc,
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

pub fn init_clickgui(modules: ModulesRc) -> ClickGui {
    ClickGui::new(modules)
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
    // prolly a better way to do this with hashmaps/hashsets in the future
    modules: ModulesRc,
}

impl ClickGui {
    pub fn new(modules: ModulesRc) -> Self {
        Self {
            modules,
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

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            for (i, module) in self.modules.iter().enumerate() {
                // need to push ids because it was reusing ids otherwise and breaking stuff
                ui.push_id(i, |ui| {
                    ui.add(module_widget(&module.borrow()));
                });
            }
        });
    
        // set the correct context
        unsafe {
            let hdc = *CLICKGUI_HDC.get().unwrap();
            let context = CLICKGUI_CONTEXT.get_mut().unwrap();
            wglMakeCurrent(hdc, *context);
        }
        ctx.request_repaint();  // repaint it because otherwise it wouldn't work i forget why this is needed but it is. but it also breaks things. so idk.
    }
}
