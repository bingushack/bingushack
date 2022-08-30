use crate::client::mapping::MappingsManager;
use std::rc::Rc;
use jni::JNIEnv;
use crate::client::BoxedBingusSetting;

// todo make this all a nice big proc macro
pub trait BingusModule {
    fn new_boxed() -> Box<dyn BingusModule> where Self: Sized;

    fn tick(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn on_load(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn on_unload(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn on_enable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn on_disable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>);

    fn get_settings_mut(&mut self) -> &mut Vec<super::BoxedBingusSetting>;

    fn get_enabled(&self) -> BoxedBingusSetting;

    fn get_enabled_mut(&mut self) -> &mut BoxedBingusSetting;

    fn to_name(&self) -> &'static str;
}
