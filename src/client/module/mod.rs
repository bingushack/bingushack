pub mod modules;
mod module;

pub use self::module::BingusModule;

use super::BoxedBingusModule;
use super::setting::{
    BingusSettings,
    SettingValue,
};

type SettingType = ::std::rc::Rc<::std::cell::RefCell<BingusSettings>>;
