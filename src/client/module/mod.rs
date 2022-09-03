pub mod modules;
mod module;

pub use self::module::BingusModule;

use super::BoxedBingusModule;
use super::setting::{
    BingusSettings,
    SettingValue,
};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub type SettingType = Arc<Mutex<RefCell<BingusSettings>>>;
pub type AllSettingsType =  Arc<Mutex<RefCell<Vec<Rc<RefCell<BingusSettings>>>>>>;
