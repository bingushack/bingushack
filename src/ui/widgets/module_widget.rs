use eframe::egui;
use crate::client::module::BingusModule;

use super::toggle;

fn module_ui(ui: &mut egui::Ui, module: &mut impl BingusModule) -> egui::Response {
    // let module_clone = module.clone();

    let desired_size = ui.spacing().interact_size.y * egui::vec2(5.0, 2.0);

    // response is mut on purpose
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        ui.horizontal(|ui| {
            ui.add(toggle(&mut (**module.get_enabled_mut()).get_value().try_into().unwrap()));

            ui.collapsing(module.to_name(), |_ui| {
                for _setting in module.get_settings_mut().iter_mut() {
                    // can't add settings because i haven't made them yet lol
                }
            });
        });
    }

    response
}

pub fn module_widget(module: &mut impl BingusModule) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| module_ui(ui, module)
}