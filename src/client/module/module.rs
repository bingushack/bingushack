use crate::{client::mapping::MappingsManager, RENDER_MANAGER, log_to_file, CLIENT_MANAGER};
use std::{rc::Rc, sync::atomic::AtomicPtr};

use super::{AllSettingsType, SettingType};
use crate::client::BoxedBingusModule;
use jni::JNIEnv;
use winapi::shared::windef::HDC__;

// todo make this all a nice big proc macro
// atm it is a trait which each module implements
pub trait BingusModule {
    // where you add the fields to the struct including enabled or not and settings
    fn new_boxed(env: &'static Rc<JNIEnv>, mappings_manager: &'static Rc<MappingsManager>) -> BoxedBingusModule
    where
        Self: Sized;

    fn tick(&self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {}

    fn if_enabled_tick(&self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {
        if self.get_enabled() {
            self.tick(env, mappings_manager);
        }
    }

    fn add_render_method_to_manager(module: &'static dyn BingusModule)
    where
        Self: Sized
    {
        fn foo(module: &dyn BingusModule) -> Box<dyn Fn() -> () + 'static> {
            let module = unsafe { std::mem::transmute::<&dyn BingusModule, &dyn BingusModule>(module) };
            Box::new(move || module.render_event())
        }
        let callback = foo(module);
        unsafe {
            RENDER_MANAGER.get_mut().unwrap().add_render_method(&callback, module.get_enabled_setting());
        }
    }

    fn add_client_tick_method_to_manager(module: &'static dyn BingusModule, env: &'static Rc<JNIEnv>, mappings_manager: &'static Rc<MappingsManager>)
    where
        Self: Sized
    {
        fn foo(module: &'static dyn BingusModule, env: &'static Rc<JNIEnv>, mappings_manager: &'static Rc<MappingsManager>) -> Box<dyn Fn() -> () + 'static> {
            Box::new(move || BingusModule::if_enabled_tick(module, Rc::clone(&env), Rc::clone(&mappings_manager)))
        }
        let callback = foo(module, env, mappings_manager);
        unsafe {
            CLIENT_MANAGER.get_mut().unwrap().add_callback(callback, module.get_enabled_setting());
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
