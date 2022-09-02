use std::cell::{
    RefCell,
    Ref,
    RefMut,
};
use std::rc::Rc;
use crate::client::setting::{
    BingusSettings,
    SettingValue,
    BooleanSetting,
};

use eframe::egui;

fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);

    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if response.clicked() {
        *on = !*on;
    }


    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);

        let visuals = ui.style().interact_selectable(&response, *on);


        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();

        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);

        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}


pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}