use crate::{client::mapping::MappingsManager, RENDER_MANAGER, log_to_file, CLIENT_MANAGER, managers::ClientCallbackType};
use std::{rc::Rc, sync::{atomic::AtomicPtr, Arc}};

use super::{AllSettingsType, SettingType, modules::ModulesEnum};
use crate::client::BoxedBingusModule;
use enum_dispatch::enum_dispatch;
use jni::JNIEnv;
use winapi::shared::windef::HDC__;

// todo make this all a nice big proc macro
// atm it is a trait which each module implements
#[enum_dispatch]
pub trait BingusModule {
    // where you add the fields to the struct including enabled or not and settings
    fn init(&mut self);

    fn tick(&self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {}

    fn if_enabled_tick(&self, env: &'static Rc<JNIEnv>, mappings_manager: &'static Rc<MappingsManager>) {
        if self.get_enabled() {
            self.tick(Rc::clone(env), Rc::clone(mappings_manager));
        }
    }

    fn render_event(&self) {}

    fn on_load(&self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {}

    fn on_unload(&self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {}

    fn on_enable(&self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {}

    fn on_disable(&self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {}

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

pub trait Newable {
    fn new() -> Self;
}

pub fn add_render_method_to_manager(md: &'static ModulesEnum) {
    let callback = _render_foo(md);
    unsafe {
        RENDER_MANAGER.get_mut().unwrap().add_render_method(&callback, md.get_enabled_setting());
    }
}

pub fn add_client_callback(
    md: &'static ModulesEnum,
    env: &'static Rc<JNIEnv>,
    mappings_manager: &'static Rc<MappingsManager>,
    setting: SettingType,
    client_callback_type: ClientCallbackType,
) {
    let callback = _client_foo(md, env, mappings_manager, client_callback_type);
    unsafe {
        CLIENT_MANAGER.get_mut().unwrap().add_callback(callback, setting, client_callback_type);
    }
}

fn _render_foo(md: &'static ModulesEnum) -> Box<dyn Fn() -> () + 'static> {
    Box::new(move || md.render_event())
}

fn _client_foo(md: &'static ModulesEnum, env: &'static Rc<JNIEnv>, mappings_manager: &'static Rc<MappingsManager>, client_callback_type: ClientCallbackType) -> Box<dyn Fn() -> () + 'static> {
    match client_callback_type {
        ClientCallbackType::Tick => Box::new(move || md.if_enabled_tick(env, mappings_manager)),
        ClientCallbackType::Load => Box::new(move || md.on_load(Rc::clone(env), Rc::clone(mappings_manager))),
    }
}

/*
fn add_render_method_to_manager(module: ModulesEnum) {
    fn foo(module: ModulesEnum) -> Box<dyn Fn() -> () + 'static> {
        Box::new(move || module.render_event())
    }
    let callback = foo(module);
    unsafe {
        RENDER_MANAGER.get_mut().unwrap().add_render_method(&callback, module.get_enabled_setting());
    }
}

fn add_client_tick_method_to_manager(module: ModulesEnum, env: &'static Rc<JNIEnv>, mappings_manager: &'static Rc<MappingsManager>) {
    fn foo(module: ModulesEnum, env: &'static Rc<JNIEnv>, mappings_manager: &'static Rc<MappingsManager>) -> Box<dyn Fn() -> () + 'static> {
        Box::new(move || module.if_enabled_tick(env, mappings_manager))
    }
    let callback = foo(module, env, mappings_manager);
    unsafe {
        CLIENT_MANAGER.get_mut().unwrap().add_callback(callback, module.get_enabled_setting(), crate::managers::ClientCallbackType::Tick);
    }
}

fn add_module_load_method_to_manager(module: ModulesEnum, env: &'static Rc<JNIEnv>, mappings_manager: &'static Rc<MappingsManager>) {
    fn foo(module: ModulesEnum, env: &'static Rc<JNIEnv>, mappings_manager: &'static Rc<MappingsManager>) -> Box<dyn Fn() -> () + 'static> {
        Box::new(move || module.on_load(Rc::clone(&env), Rc::clone(&mappings_manager)))
    }
    let callback = foo(module, env, mappings_manager);
    unsafe {
        CLIENT_MANAGER.get_mut().unwrap().add_callback(callback, module.get_enabled_setting(), crate::managers::ClientCallbackType::Load);
    }
}
*/