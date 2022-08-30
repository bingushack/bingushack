pub enum ClickGuiMessage {
    RunModule(Box<dyn crate::client::module::BingusModule>),
}
