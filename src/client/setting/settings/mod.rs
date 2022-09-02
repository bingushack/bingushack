// maybe find a way to do this with macros and/or const generics

mod boolean_setting;
mod float_setting;

pub use self::boolean_setting::*;
pub use self::float_setting::*;
use crate::client::setting::{
    BingusSetting,
    SettingValue,
};


// todo MACRO holy shit
#[derive(Debug, Clone)]
pub enum BingusSettings {
    BooleanSetting(BooleanSetting),
    FloatSetting(FloatSetting),
}

impl BingusSettings {
    pub fn get_value(&self) -> SettingValue {
        match self {
            // todo MACRO!!!!!!!!!!!!!!!!!!
            BingusSettings::BooleanSetting(setting) => setting.get_value(),
            BingusSettings::FloatSetting(setting) => setting.get_value(),
        }
    }

    pub fn set_value(&mut self, value: SettingValue) {
        match self {
            // todo MACRO!!!!!!!!!!!!!!!!!!
            BingusSettings::BooleanSetting(setting) => setting.set_value(value),
            BingusSettings::FloatSetting(setting) => setting.set_value(value),
        }
    }

    pub fn get_name(&self) -> &String {
        match self {
            // todo MACRO!!!!!!!!!!!!!!!!!!
            BingusSettings::BooleanSetting(setting) => setting.get_name(),
            BingusSettings::FloatSetting(setting) => setting.get_name(),
        }
    }

    pub fn get_bool_mut(&mut self) -> &mut bool {
        match self {
            BingusSettings::BooleanSetting(setting) => setting.get_value_mut(),
            _ => panic!("get_bool_mut called on a non-boolean setting"),
        }
    }

    pub fn get_float_mut(&mut self) -> &mut f64 {
        match self {
            BingusSettings::FloatSetting(setting) => setting.get_value_mut(),
            _ => panic!("get_float_mut called on a non-float setting"),
        }
    }
}


// todo I NEED A MACRO FOR THIS
impl TryInto<BooleanSetting> for BingusSettings {
    type Error = ();
    fn try_into(self) -> Result<BooleanSetting, Self::Error> {
        match self {
            BingusSettings::BooleanSetting(setting) => Ok(setting),
            _ => Err(()),
        }
    }
}

impl TryInto<FloatSetting> for BingusSettings {
    type Error = ();
    fn try_into(self) -> Result<FloatSetting, Self::Error> {
        match self {
            BingusSettings::FloatSetting(setting) => Ok(setting),
            _ => Err(()),
        }
    }
}
