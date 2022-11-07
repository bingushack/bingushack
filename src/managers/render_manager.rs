use crate::client::module::SettingType;

pub struct RenderManager {
    callbacks: Vec<(&'static dyn Fn(), SettingType)>,
}

impl RenderManager {
    pub fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }

    pub fn add_render_method<'a>(&mut self, method: &'a dyn Fn(), enabled: SettingType) {
        // transmute to static lifetime
        let method: &'static dyn Fn() = unsafe { std::mem::transmute(method) };
        self.callbacks.push((method, enabled));
    }

    pub fn get_render_methods(&self) -> &Vec<(&dyn Fn(), SettingType)> {
        &self.callbacks
    }
}