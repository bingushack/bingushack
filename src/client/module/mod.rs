mod module;
pub mod modules;

pub use self::module::BingusModule;

use super::{
    setting::{BingusSettings, SettingValue},
    BoxedBingusModule,
};

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub type SettingType = Arc<Mutex<RefCell<BingusSettings>>>;
pub type AllSettingsType = Arc<Mutex<RefCell<Vec<Rc<RefCell<BingusSettings>>>>>>;
