mod autototem;
mod testmodule;

pub use self::autototem::*;
pub use self::testmodule::*;

use super::{
    BingusSetting,
    BingusModule,
    BoxedBingusSetting,
    BoxedBingusModule,
    SettingValue
};
use crate::client::mapping::MemTrait;
use super::SettingType;