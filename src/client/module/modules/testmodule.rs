// only there on debug builds. doesn't really do anything atm



use super::{
    AllSettingsType, BingusModule, BingusSettings, BoxedBingusModule, SettingType, SettingValue,
};
use crate::client::{
    mapping::MappingsManager,
    setting::{BooleanSetting, FloatSetting}, module::module::Newable,
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
    enabled: Option<SettingType>,

    settings: Option<AllSettingsType>,
}

// derive macro when?
impl Newable for TestModule {
    fn new() -> Self {
        Self {
            enabled: None,
            settings: None,
        }
    }
}

impl BingusModule for TestModule {
    fn init(&mut self) {
        self.enabled = Some(Arc::new(Mutex::new(RefCell::new(BingusSettings::BooleanSetting(
            BooleanSetting::new(SettingValue::from(false), "enabled"),
        )))));
        self.settings = Some(Arc::new(Mutex::new(RefCell::new(vec![Rc::new(RefCell::new(
            BingusSettings::FloatSetting(FloatSetting::new(
                SettingValue::from(0.0),
                "float",
                0.0..=100.0,
            )),
        ))]))));
    }

    fn tick(&self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {
        let minecraft_client = mappings_manager.get("MinecraftClient").unwrap();
        apply_object!(
            minecraft_client,
            call_method_or_get_field!(env, minecraft_client, "getInstance", true, &[]).unwrap().l().unwrap()
        );
    }

    fn get_all_settings(&self) -> AllSettingsType {
        Arc::clone(self.settings.as_ref().unwrap())
    }

    fn get_enabled_setting(&self) -> SettingType {
        Arc::clone(self.enabled.as_ref().unwrap())
    }

    fn to_name(&self) -> String {
        "TestModule".to_string()
    }
}
