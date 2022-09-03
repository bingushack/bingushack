use super::{
    AllSettingsType, BingusModule, BingusSettings, BoxedBingusModule, SettingType, SettingValue,
};
use crate::client::{
    mapping::MappingsManager,
    setting::{BooleanSetting, FloatSetting},
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
    fn new_boxed() -> BoxedBingusModule {
        Box::new(Self {
            enabled: Arc::new(Mutex::new(RefCell::new(BingusSettings::BooleanSetting(
                BooleanSetting::new(SettingValue::from(false), "enabled"),
            )))),
            settings: Arc::new(Mutex::new(RefCell::new(vec![Rc::new(RefCell::new(
                BingusSettings::FloatSetting(FloatSetting::new(
                    SettingValue::from(0.0),
                    "float",
                    0.0..=5.0,
                )),
            ))]))),
        })
    }

    fn tick(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

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
