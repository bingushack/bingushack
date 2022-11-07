// only there on debug builds. doesn't really do anything atm



use super::{
    AllSettingsType, BingusModule, BingusSettings, BoxedBingusModule, SettingType, SettingValue,
};
use crate::client::{
    mapping::MappingsManager,
    setting::{BooleanSetting, FloatSetting},
};
use crate::{
    apply_object,
    call_method_or_get_field,
};
use jni::JNIEnv;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub struct TestModule {
    enabled: SettingType,

    settings: AllSettingsType,
}

impl BingusModule for TestModule {
    fn new_boxed(env: &'static Rc<JNIEnv>, mappings_manager: &'static Rc<MappingsManager>) -> BoxedBingusModule {
        let to_ret = Self {
            enabled: Arc::new(Mutex::new(RefCell::new(BingusSettings::BooleanSetting(
                BooleanSetting::new(SettingValue::from(false), "enabled"),
            )))),
            settings: Arc::new(Mutex::new(RefCell::new(vec![Rc::new(RefCell::new(
                BingusSettings::FloatSetting(FloatSetting::new(
                    SettingValue::from(0.0),
                    "float",
                    0.0..=100.0,
                )),
            ))]))),
        };

        let to_ret = Box::new(to_ret);

        let to_ret = unsafe { std::mem::transmute::<Box<Self>, BoxedBingusModule>(to_ret) };
        <crate::client::module::modules::testmodule::TestModule as BingusModule>::add_client_tick_method_to_manager(*to_ret, env, mappings_manager);
        <crate::client::module::modules::testmodule::TestModule as BingusModule>::add_render_method_to_manager(*to_ret);
        to_ret
    }

    fn tick(&self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {
        let minecraft_client = mappings_manager.get("MinecraftClient").unwrap();
        apply_object!(
            minecraft_client,
            call_method_or_get_field!(env, minecraft_client, "getInstance", true, &[]).unwrap().l().unwrap()
        );
    }

    fn on_load(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn on_unload(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn on_enable(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn on_disable(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn get_all_settings(&self) -> AllSettingsType {
        Arc::clone(&self.settings)
    }

    fn get_enabled_setting(&self) -> SettingType {
        Arc::clone(&self.enabled)
    }

    fn to_name(&self) -> String {
        "TestModule".to_string()
    }
}
