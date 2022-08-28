use crate::client::Modules;

#[derive(Debug)]
pub struct EnabledSetting {
    module: Modules,
    enabled: bool,
}

impl EnabledSetting {
    pub fn new(module: Modules, enabled: bool) -> Self {
        Self {
            module,
            enabled,
        }
    }

    pub fn get_enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_module(&self) -> Modules {
        self.module
    }
}
