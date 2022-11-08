use crate::client::module::SettingType;

pub struct RenderManager {
    callbacks: Vec<(Box<dyn Fn()>, SettingType)>,
}

impl RenderManager {
    pub fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }

    pub fn add_render_method<'a>(&mut self, method: Box<dyn Fn()>, enabled: SettingType) {
        self.callbacks.push((method, enabled));
    }

    pub fn call_render_callbacks(&self) {
        for (callback, enabled) in &self.callbacks {
            if enabled.lock()
                .unwrap()
                .borrow()
                .get_value()
                .try_into()
                .unwrap()
            {
                callback();
            }
        }
    }
}