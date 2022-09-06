use crate::client::{module::BingusModule, setting::*};
use eframe::egui;
use std::{cell::RefMut, rc::Rc};

use super::{toggle, DoubleSlider};

fn module_ui<'a>(ui: &mut egui::Ui, module: &'a Box<dyn BingusModule>) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(5.0, 2.0);

    // response is mut on purpose
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        ui.horizontal(|ui| {
            ui.add(toggle(
                module
                    .get_enabled_setting()
                    .lock()
                    .unwrap()
                    .borrow_mut()
                    .get_bool_mut()
                    .get_value_mut(),
            ));

            ui.collapsing(module.to_name(), |ui| {
                let settings_mutex = module.get_all_settings();
                let mut to_unleak = settings_mutex.lock().unwrap();
                let first_leaked = RefMut::leak(to_unleak.borrow_mut());
                for setting in (*first_leaked).iter_mut() {
                    let second_leaked = RefMut::leak((*setting).borrow_mut());
                    match second_leaked {
                        BingusSettings::BooleanSetting(_) => {
                            ui.label(second_leaked.get_name());
                            ui.add(toggle(second_leaked.get_bool_mut().get_value_mut()));
                        }
                        BingusSettings::FloatSetting(_) => {
                            ui.label(second_leaked.get_name());
                            let range = second_leaked.get_float_mut().get_range();
                            ui.add(egui::Slider::new(
                                second_leaked.get_float_mut().get_value_mut(),
                                range,
                            ));
                        }
                        BingusSettings::RangeSetting(_) => {
                            ui.label(second_leaked.get_name());
                            let range = second_leaked.get_range();
                            let max_decimals = second_leaked.get_max_decimals();
                            let step_by = second_leaked.get_step_by();
                            let value = second_leaked.get_range_value_mut();
                            let mut double_slider = DoubleSlider::new(value, range);
                            if let Some(max_decimals) = max_decimals {
                                double_slider = double_slider.max_decimals(max_decimals);
                            }
                            if let Some(step_by) = step_by {
                                double_slider = double_slider.step_by(step_by);
                            }
                            ui.add(double_slider);
                        }
                    }
                    // undo second_leaked
                    Rc::make_mut(setting).undo_leak();
                }
                // undo first_leaked
                to_unleak.undo_leak();
            });
        });
    }

    response
}

pub fn module_widget<'a>(module: &'a Box<dyn BingusModule>) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| module_ui(ui, module)
}
