pub mod modules;
mod module;

pub use self::module::BingusModule;

use super::{
    BoxedBingusSetting,
    BoxedBingusModule,
};
use super::setting::{
    BingusSetting,
    SettingValue
};

type SettingType = ::std::rc::Rc<::std::cell::RefCell<BoxedBingusSetting>>;
