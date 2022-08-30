use std::rc::Rc;
use std::cell::RefCell;

pub enum ClickGuiMessage {
    RunModule(Rc<RefCell<Box<dyn crate::client::module::BingusModule>>>),
}
