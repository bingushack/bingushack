mod autototem;
mod testmodule;
mod triggerbot;
mod esp;

use enum_dispatch::enum_dispatch;

pub use self::{autototem::*, testmodule::*, triggerbot::*, esp::*};
use crate::{
    MappingsManager,
    JNIEnv,
    managers::ClientCallbackType
};
use std::rc::Rc;

use super::{
    AllSettingsType, BingusModule, BingusSettings, BoxedBingusModule, SettingType, SettingValue,
};

#[enum_dispatch(BingusModule)]
pub enum ModulesEnum {
    AutoTotem(AutoTotem),
    Esp(Esp),
    TestModule(TestModule),
    Triggerbot(Triggerbot),
}
