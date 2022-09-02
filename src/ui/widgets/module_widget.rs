use eframe::egui;
use crate::client::module::BingusModule;
use crate::client::setting::*;
use std::cell::{
    RefMut,
    RefCell,
};
use std::rc::Rc;

use super::toggle;

fn module_ui<'a>(ui: &mut egui::Ui, module: &'a Box<dyn BingusModule>) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(5.0, 2.0);

    // response is mut on purpose
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        ui.horizontal(|ui| {
            ui.add(toggle(
                module.get_enabled_ref_cell().borrow_mut().get_bool_mut(),
            ));

            ui.collapsing(module.to_name(), |ui| {
                for setting in &*module.get_settings_ref_cell() {
                    match RefMut::leak(setting.clone().borrow_mut()) {
                        BingusSettings::BooleanSetting(setting) => {
                            ui.label(setting.get_name());
                            ui.add(toggle(setting.get_value_mut()));
                        },
                        BingusSettings::FloatSetting(setting) => {
                            ui.label(setting.get_name());
                            let range = setting.get_range();
                            ui.add(egui::Slider::new(
                                setting.get_value_mut(),
                                range,
                            ));
                        },
                    }
                    // idk if i need to undo leak bc it's dropped
                }
            });
        });
    }

    response
}

pub fn module_widget<'a>(module: &'a Box<dyn BingusModule>) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| module_ui(ui, module)
}