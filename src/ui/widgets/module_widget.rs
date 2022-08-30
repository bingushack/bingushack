use eframe::egui;
use crate::client::module::BingusModule;
use std::borrow::BorrowMut;

use super::toggle;

fn module_ui<'a>(ui: &mut egui::Ui, module: &'a Box<dyn BingusModule>) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(5.0, 2.0);

    // response is mut on purpose
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        ui.horizontal(|ui| {
            ui.add(toggle(&mut (*module.get_enabled_ref_cell()).borrow_mut().get_value().try_into().unwrap()));

            ui.collapsing(module.to_name(), |_ui| {
                for _setting in &*module.get_settings_ref_cell() {
                    // can't add settings because i haven't made them yet lol
                    let _ = (**_setting).borrow_mut();
                }
            });
        });
    }

    response
}

pub fn module_widget<'a>(module: &'a Box<dyn BingusModule>) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| module_ui(ui, module)
}