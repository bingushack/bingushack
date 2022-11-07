use crate::{client::mapping::MappingsManager, RENDER_MANAGER, log_to_file};
use std::{rc::Rc, sync::atomic::AtomicPtr};

use super::{AllSettingsType, SettingType};
use crate::client::BoxedBingusModule;
use jni::JNIEnv;
use winapi::shared::windef::HDC__;

// todo make this all a nice big proc macro
// atm it is a trait which each module implements
pub trait BingusModule {
    // where you add the fields to the struct including enabled or not and settings
    fn new_boxed() -> BoxedBingusModule
    where
        Self: Sized;

    fn tick(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {}

    fn add_render_method_to_manager(module: &Self)
    where
        Self: Sized
    {
        let callback = || Self::render_event(module);
        unsafe {
            RENDER_MANAGER.get_mut().unwrap().add_render_method(&callback, Self::get_enabled_setting(module));
        }
    }

    fn render_event(&self) {}

    fn on_load(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {}

    fn on_unload(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {}

    fn on_enable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {}

    fn on_disable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {}

    fn get_all_settings(&self) -> AllSettingsType;

    fn get_enabled_setting(&self) -> SettingType;

    fn get_enabled(&self) -> bool {
        self
            .get_enabled_setting()
            .lock()
            .unwrap()
            .borrow()
            .get_value()
            .try_into()
            .unwrap()
    }

    // the name that will be diplayed on the clickgui
    fn to_name(&self) -> String;
}
