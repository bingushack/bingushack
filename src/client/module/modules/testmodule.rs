use super::{
    BingusSetting,
    SettingValue,
    BingusModule,
    BoxedBingusSetting,
    BoxedBingusModule,
    MemTrait,
    SettingType,
};

pub struct TestModule {
    enabled: SettingType,
    settings: Rc<Vec<SettingType>>,

    name: String,
}

impl BingusModule for TestModule {
    fn new_boxed(name: &str) -> BoxedBingusModule {
        Box::new(
            Self {
                enabled: Rc::new(RefCell::new(BooleanSetting::new_boxed(SettingValue::from(false)))),
                settings: Rc::new(vec![]),

                name: name.to_string(),
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

    fn to_name(&self) -> &'static str {
        self.name.as_str()
    }
}
