use std::{cell::RefCell, rc::Rc};

pub enum ClickGuiMessage {
    RunModule(Rc<RefCell<Box<dyn crate::client::module::BingusModule>>>),
}
