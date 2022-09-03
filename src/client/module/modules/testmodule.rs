use super::{
    BingusSettings,
    SettingValue,
    BingusModule,
    BoxedBingusModule,
    MemTrait,
    SettingType,
    AllSettingsType,
};
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use crate::client::mapping::MappingsManager;
use jni::JNIEnv;
use std::sync::Mutex;
use crate::client::setting::{
    BooleanSetting,
    FloatSetting,
};

pub struct TestModule {
    enabled: SettingType,
    settings: AllSettingsType,
}

impl BingusModule for TestModule {
    fn new_boxed() -> BoxedBingusModule {
        Box::new(
            Self {
                enabled: Arc::new(Mutex::new(RefCell::new(BingusSettings::BooleanSetting(BooleanSetting::new(SettingValue::from(false), "enabled"))))),
                settings: Arc::new(Mutex::new(RefCell::new(vec![
                    Rc::new(RefCell::new(BingusSettings::FloatSetting(FloatSetting::new(SettingValue::from(0.0), "float", 0.0..=5.0)))),
                ]))),
            }
        )
    }

    fn tick(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn on_load(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn on_unload(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn on_enable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn on_disable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

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
