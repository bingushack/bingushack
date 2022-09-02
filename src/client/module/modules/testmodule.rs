use super::{
    BingusSettings,
    SettingValue,
    BingusModule,
    BoxedBingusModule,
    MemTrait,
    SettingType,
};
use std::rc::Rc;
use std::cell::RefCell;
use crate::client::mapping::MappingsManager;
use jni::JNIEnv;
use crate::client::setting::BooleanSetting;

pub struct TestModule {
    enabled: Rc<RefCell<BingusSettings>>,
    settings: Rc<Vec<Rc<RefCell<BingusSettings>>>>,
}

impl BingusModule for TestModule {
    fn new_boxed() -> BoxedBingusModule {
        Box::new(
            Self {
                enabled: Rc::new(RefCell::new(BingusSettings::BooleanSetting(BooleanSetting::new(SettingValue::from(false), "enabled")))),
                settings: Rc::new(vec![]),
            }
        )
    }

    fn tick(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn on_load(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn on_unload(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn on_enable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn on_disable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn get_settings_ref_cell(&self) -> Rc<Vec<Rc<RefCell<BingusSettings>>>> {
        Rc::clone(&self.settings)
    }

    fn get_enabled_ref_cell(&self) -> Rc<RefCell<BingusSettings>> {
        Rc::clone(&self.enabled)
    }

    fn to_name(&self) -> String {
        "TestModule".to_string()
    }
}
