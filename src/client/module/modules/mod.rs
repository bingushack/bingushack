mod autototem;
mod testmodule;

pub use self::autototem::*;
pub use self::testmodule::*;

use super::{
    BingusModule,
    BoxedBingusModule,
    BingusSettings,
    SettingValue
};
use crate::client::mapping::MemTrait;
use super::SettingType;