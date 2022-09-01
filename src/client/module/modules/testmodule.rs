use super::{
    BingusSetting,
    SettingValue,
    BingusModule,
    BoxedBingusSetting,
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
    enabled: SettingType,
    settings: Rc<Vec<SettingType>>,
}

impl BingusModule for TestModule {
    fn new_boxed() -> BoxedBingusModule {
        Box::new(
            Self {
                enabled: Rc::new(RefCell::new(BooleanSetting::new_boxed(SettingValue::from(false)))),
                settings: Rc::new(vec![]),
            }
        )
    }

    fn tick(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn on_load(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn on_unload(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn on_enable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn on_disable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {  }

    fn get_settings_ref_cell(&self) -> Rc<Vec<SettingType>> {
        Rc::clone(&self.settings)
    }

    fn get_enabled_ref_cell(&self) -> SettingType {
        Rc::clone(&self.enabled)
    }

    fn to_name(&self) -> String {
        // shittiest way to make a non-crytpographicly secure number
        (((&vec![2, 3] as *const Vec<i32>) as usize) % 10000).to_string()
    }
}
