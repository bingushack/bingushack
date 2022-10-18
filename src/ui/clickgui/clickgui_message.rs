use std::{cell::RefCell, rc::Rc};

// messages to send from the ClickGui to the Client
pub enum ClickGuiMessage {
    RunModule(Rc<RefCell<Box<dyn crate::client::module::BingusModule>>>),
    RunRenderEvent,
}
