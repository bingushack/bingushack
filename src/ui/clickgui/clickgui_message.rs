use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicPtr};

use winapi::shared::windef::{HDC, HDC__};

// messages to send from the ClickGui to the Client
pub enum ClickGuiMessage {
    RunModule(Rc<RefCell<Box<dyn crate::client::module::BingusModule>>>),
    RunRenderEvent,
}
