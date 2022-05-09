use std::sync::mpsc::{Receiver, Sender};
use egui_winit::egui;
use egui_winit::egui::Context;
use crate::ui::util::{App, NativeOptions};
use crate::ui::run;

pub fn init_debug_console() -> Sender<String> {
    let (tx, rx) = std::sync::mpsc::channel();
    tx
}

pub fn run_debug_console() {
    let options = NativeOptions::default();
    run(
        "My egui App",
        &options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut crate::ui::util::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
