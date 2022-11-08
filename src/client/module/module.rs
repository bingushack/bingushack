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
    // where you add the fields to the struct including enabled or not and settings
    fn new() -> Self;
}

pub fn add_render_method_to_manager(md: ModulesEnum) -> &'static ModulesEnum {
    let (callback, md) = _render_foo(&md);
    unsafe {
        RENDER_MANAGER.get_mut().unwrap().add_render_method(callback, md.get_enabled_setting());
    }
    md
}

pub fn add_client_callback(
    md: &'static ModulesEnum,
    env: &'static Rc<JNIEnv>,
    mappings_manager: &'static Rc<MappingsManager>,
    setting: SettingType,
    client_callback_type: ClientCallbackType,
) -> &'static ModulesEnum {
    let (callback, md) = _client_foo(&md, env, mappings_manager, client_callback_type);
    unsafe {
        CLIENT_MANAGER.get_mut().unwrap().add_callback(callback, setting, client_callback_type);
    }
    md
}

fn _render_foo<'a>(md: &'a ModulesEnum) -> (Box<dyn Fn()>, &'static ModulesEnum) {
    // transmute md to static lifetime
    let md = unsafe { std::mem::transmute::<&ModulesEnum, &'static ModulesEnum>(md) };
    let mth = Box::new(move || md.render_event());
    // transmute mth to static lifetime
    (mth, md)
}

fn _client_foo<'a>(
    md: &'a ModulesEnum,
    env: &'static Rc<JNIEnv>,
    mappings_manager: &'static Rc<MappingsManager>,
    client_callback_type: ClientCallbackType
) -> (Box<dyn Fn()>, &'static ModulesEnum) {
    // transmute md to static lifetime
    let md = unsafe { std::mem::transmute::<&ModulesEnum, &'static ModulesEnum>(md) };
    (
        match client_callback_type {
            ClientCallbackType::Tick => Box::new(move || md.if_enabled_tick(env, mappings_manager)),
            ClientCallbackType::Load => Box::new(move || md.on_load(Rc::clone(env), Rc::clone(mappings_manager))),
        },
        md
    )
}
