use crate::client::mapping::MappingsManager;
use std::rc::Rc;
use jni::JNIEnv;
use crate::client::RcBoxedBingusSetting;

// todo make this all a nice big proc macro
pub trait BingusModule {
    fn new_boxed() -> Box<dyn BingusModule> where Self: Sized;

    fn tick(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn on_load(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn on_unload(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn on_enable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn on_disable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn get_settings_mut(&mut self) -> &mut Vec<super::RcBoxedBingusSetting>;

    fn get_enabled(&self) -> RcBoxedBingusSetting;

    fn get_enabled_mut(&mut self) -> &mut RcBoxedBingusSetting;

    fn to_name(&self) -> &'static str;
}
