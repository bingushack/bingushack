type SettingsType = Vec<()>;


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Modules {
    AutoTotem(bool, SettingsType),  // will eventually contain settings
}

impl Modules {
    pub fn get_settings_mut(&mut self) -> &mut SettingsType {
        match self {
            Modules::AutoTotem(_, settings) => settings,
        }
    }

    pub fn get_enabled(&self) -> bool {
        match self {
            Modules::AutoTotem(enabled, _) => *enabled,
        }
    }

    pub fn get_enabled_mut(&mut self) -> &mut bool {
        match self {
            Modules::AutoTotem(enabled, _) => enabled,
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            Modules::AutoTotem(_, _) => "AutoTotem",
        }
    }
}
