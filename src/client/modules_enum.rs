type SettingsType = Vec<()>;


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Modules {
    AutoTotem(bool, SettingsType),  // will eventually contain settings

    Unused,  // for pattern-matching purposes
}

impl Modules {
    pub fn get_settings(&self) -> &SettingsType {
        match self {
            Modules::AutoTotem(_, settings) => settings,
            _ => unimplemented!(),
        }
    }

    pub fn get_settings_mut(&mut self) -> &mut SettingsType {
        match self {
            Modules::AutoTotem(_, settings) => settings,
            _ => unimplemented!(),
        }
    }

    pub fn get_enabled(&self) -> bool {
        match self {
            Modules::AutoTotem(enabled, _) => *enabled,
            _ => unimplemented!(),
        }
    }

    pub fn get_enabled_mut(&mut self) -> &mut bool {
        match self {
            Modules::AutoTotem(enabled, _) => enabled,
            _ => unimplemented!(),
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            Modules::AutoTotem(_, _) => "AutoTotem",
            _ => unimplemented!(),
        }
    }
}
