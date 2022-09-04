mod autototem;
mod testmodule;
mod triggerbot;

pub use self::{autototem::*, testmodule::*, triggerbot::*};

use super::{
    AllSettingsType, BingusModule, BingusSettings, BoxedBingusModule, SettingType, SettingValue,
};
