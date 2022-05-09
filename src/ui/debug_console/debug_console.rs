use std::sync::mpsc::{Receiver, Sender};
use egui_winit::egui;
use egui_winit::egui::RichText;
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
    my_string: String,
    my_boolean: bool,
    my_f32: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            my_string: "button text- edit me!".to_owned(),
            my_boolean: false,
            my_f32: 50.0,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut crate::ui::util::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.ctx().set_visuals(egui::Visuals::dark());  // might not be needed anymore


            ui.label(RichText::new("Large and underlined").size(self.my_f32).underline());
            ui.hyperlink("https://github.com/emilk/egui");
            ui.text_edit_singleline(&mut self.my_string);
            if ui.button((&mut self.my_string).as_str()).clicked() { }
            ui.add(egui::Slider::new(&mut self.my_f32, 0.0..=100.0));
            ui.add(egui::DragValue::new(&mut self.my_f32));

            ui.checkbox(&mut self.my_boolean, "Checkbox");

            /*ui.horizontal(|ui| {
                ui.radio_value(&mut my_enum, MyEnum::First, "First");
                ui.radio_value(&mut my_enum, MyEnum::Second, "Second");
                ui.radio_value(&mut my_enum, MyEnum::Third, "Third");
            });*/

            ui.separator();

            ui.collapsing("Click to see what is hidden!", |ui| {
                ui.label("Not much, as it turns out");
            });
        });
    }
}

/*#[derive(PartialEq)]
enum MyEnum { First, Second, Third }
 */