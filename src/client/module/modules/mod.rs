mod autototem;
mod testmodule;

pub use self::{autototem::*, testmodule::*};

use super::{
    AllSettingsType, BingusModule, BingusSettings, BoxedBingusModule, SettingType, SettingValue,
};
use crate::client::mapping::MemTrait;
