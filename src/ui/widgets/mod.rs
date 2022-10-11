// most of the info you'd need for this can be found on the egui/eframe docs
mod module_widget;
mod toggle_switch;
mod double_slider;

pub use self::{
    module_widget::module_widget,
    toggle_switch::toggle,
    double_slider::*
};
