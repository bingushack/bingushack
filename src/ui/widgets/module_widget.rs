use eframe::egui;
use crate::client::module::BingusModule;
use crate::client::setting::*;
use std::cell::{
    RefMut,
    Ref,
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
                module.get_enabled_ref_cell().borrow_mut().get_bool_mut().get_value_mut(),
            ));

            ui.collapsing(module.to_name(), |ui| {
                let mut to_unleak = module.get_settings_ref_cell();
                for setting in Ref::leak(to_unleak.borrow()) {
                    let leaked = RefMut::leak(setting.borrow_mut());
                    match leaked {
                        BingusSettings::BooleanSetting(_) => {
                            ui.label(leaked.get_name());
                            ui.add(toggle(leaked.get_bool_mut().get_value_mut()));
                        },
                        BingusSettings::FloatSetting(_) => {
                            ui.label(leaked.get_name());
                            let range = leaked.get_float_mut().get_range();
                            ui.add(egui::Slider::new(
                                leaked.get_float_mut().get_value_mut(),
                                range,
                            ));
                        },
                    }
                }
            });
        });
    }

    response
}

pub fn module_widget<'a>(module: &'a Box<dyn BingusModule>) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| module_ui(ui, module)
}