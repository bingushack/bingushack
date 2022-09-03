use crate::client::mapping::MappingsManager;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::{RefCell, Ref};
use jni::JNIEnv;
use crate::client::BoxedBingusModule;
use super::{
    AllSettingsType,
    SettingType,
};

// todo make this all a nice big proc macro
pub trait BingusModule {
    fn new_boxed() -> BoxedBingusModule where Self: Sized;

    fn tick(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn on_load(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn on_unload(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn on_enable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn on_disable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn get_all_settings(&self) -> AllSettingsType;

    fn get_enabled_setting(&self) -> SettingType;

    fn to_name(&self) -> String;
}
