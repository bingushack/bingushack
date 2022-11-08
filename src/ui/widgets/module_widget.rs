// widget for a module, containing other widgets
use crate::client::{module::{BingusModule, modules::ModulesEnum}, setting::*};
use eframe::egui;
use std::{cell::{RefMut, Ref}, rc::Rc};

use super::{toggle, DoubleSlider};

// lifetime fuckery because jni object lifetimes
fn module_ui<'a>(ui: &mut egui::Ui, module: &'a Ref<Rc<&'static ModulesEnum>>) -> egui::Response {
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
                // this is the worst code ever in existence ever
                // essentially uses interior mutability for getting modules owned by something else
                // it needs to all be mutable because it is in an immediate-mode gui
                let settings_mutex = module.get_all_settings();  // gets the mutex of the settings from the module. in a mutex because it is shared between threads
                let mut to_unleak = settings_mutex.lock().unwrap();
                let first_leaked = RefMut::leak(to_unleak.borrow_mut());  // borrows the value mutably in the RefCell and leaks it to get it out of the RefMut
                for setting in (*first_leaked).iter_mut() {
                    let second_leaked = RefMut::leak((*setting).borrow_mut());  // gets the BingusSettings enum variant out of the RefCell
                    // figures out what type of setting it is and draws the ui for it
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

pub fn module_widget<'a>(module: &'a Ref<Rc<&'static ModulesEnum>>) -> impl egui::Widget + 'a {
    move |ui: &mut egui::Ui| module_ui(ui, module)
}
