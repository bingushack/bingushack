mod module;
mod modules;

pub use self::module::BingusModule;
pub use self::modules::*;

use super::BoxedBingusSetting;
use super::setting::{
    BingusSetting,
    SettingValue
};
