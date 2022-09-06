// maybe find a way to do this with macros and/or const generics

mod boolean_setting;
mod float_setting;
mod range_setting;

use std::ops::RangeInclusive;

pub use self::{boolean_setting::*, float_setting::*, range_setting::*};
use crate::client::setting::{BingusSetting, SettingValue};

// todo MACRO holy shit
#[derive(Debug, Clone)]
pub enum BingusSettings {
    BooleanSetting(BooleanSetting),
    FloatSetting(FloatSetting),
    RangeSetting(RangeSetting),
}

impl BingusSettings {
    pub fn get_value(&self) -> SettingValue {
        match self {
            // todo MACRO!!!!!!!!!!!!!!!!!!
            BingusSettings::BooleanSetting(setting) => setting.get_value(),
            BingusSettings::FloatSetting(setting) => setting.get_value(),
            BingusSettings::RangeSetting(setting) => setting.get_value(),
        }
    }

    pub fn get_name(&self) -> &String {
        match self {
            // todo MACRO!!!!!!!!!!!!!!!!!!
            BingusSettings::BooleanSetting(setting) => setting.get_name(),
            BingusSettings::FloatSetting(setting) => setting.get_name(),
            BingusSettings::RangeSetting(setting) => setting.get_name(),
        }
    }

    pub fn get_bool_mut(&mut self) -> &mut BooleanSetting {
        match self {
            BingusSettings::BooleanSetting(setting) => setting,
            _ => panic!("get_bool_mut called on a non-boolean setting"),
        }
    }

    pub fn get_float_mut(&mut self) -> &mut FloatSetting {
        match self {
            BingusSettings::FloatSetting(setting) => setting,
            _ => panic!("get_float_mut called on a non-float setting"),
        }
    }

    pub fn get_range_value_mut(&mut self) -> &mut [f64; 2] {
        match self {
            BingusSettings::RangeSetting(setting) => setting.get_value_mut(),
            _ => panic!("get_range_mut called on a non-range setting"),
        }
    }

    pub fn get_range(&self) -> RangeInclusive<f64> {
        match self {
            BingusSettings::RangeSetting(setting) => setting.get_range(),
            _ => panic!("get_range called on a non-range setting"),
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

impl TryInto<RangeSetting> for BingusSettings {
    type Error = ();
    fn try_into(self) -> Result<RangeSetting, Self::Error> {
        match self {
            BingusSettings::RangeSetting(setting) => Ok(setting),
            _ => Err(()),
        }
    }
}
