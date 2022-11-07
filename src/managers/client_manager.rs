use std::{sync::Mutex, cell::RefCell, rc::Rc};

use jni::JNIEnv;

use crate::{client::{BoxedBingusModule, module::{modules::*, BingusModule, SettingType}, MappingsManager}, log_to_file};

pub type ModulesRc = Rc<Vec<Rc<RefCell<BoxedBingusModule>>>>;

pub struct ClientManager {
    callbacks: Vec<(Box<dyn Fn()>, SettingType)>,
    jni_env: Rc<JNIEnv<'static>>,
    mappings_manager: Rc<MappingsManager<'static>>,
    modules: ModulesRc,
    enabled: bool,
}

impl ClientManager {
    pub fn new(jni_env: JNIEnv<'static>) -> Self {
        let rc_jni_env = Rc::new(jni_env);
        let mappings_manager = Rc::new(MappingsManager::new(Rc::clone(&rc_jni_env)));
        Self {
            callbacks: Vec::new(),
            jni_env: Rc::clone(&rc_jni_env),
            mappings_manager: Rc::clone(&mappings_manager),
            enabled: false,
            modules: {
                // some macros to make things easier
                //
                // this macro will make a vector containing all the modules it is given and returns it
                macro_rules! modules_maker {
                    ($($module:expr),*) => {{
                        let mut temp_vec = Vec::new();
                        $(
                            temp_vec.push(Rc::new(RefCell::new($module)));
                        )*
                        temp_vec
                    }}
                }


                // in debug mode it needs to be mutable to add the TestModule but otherwise it doesn't need to be
                #[cfg(build = "debug")]
                let mut modules;
                #[cfg(not(build = "debug"))]
                let modules;

                let static_rc_jni_env: &'static Rc<JNIEnv<'static>> = unsafe { std::mem::transmute(rc_jni_env) };
                let static_mappings_manager: &'static Rc<MappingsManager<'static>> = unsafe { std::mem::transmute(mappings_manager) };

                // add all non-debug modules
                modules = modules_maker![
                    AutoTotem::new_boxed(static_rc_jni_env, static_mappings_manager),
                    Triggerbot::new_boxed(static_rc_jni_env, static_mappings_manager),
                    Esp::new_boxed(static_rc_jni_env, static_mappings_manager)
                ];

                // if in debug add debug modules
                #[cfg(build = "debug")]
                modules.append(&mut modules_maker![
                    TestModule::new_boxed(static_rc_jni_env, static_mappings_manager)
                ]);

                Rc::new(modules)
            }
        }
    }

    pub fn get_modules(&self) -> ModulesRc {
        Rc::clone(&self.modules)
    }
    
    pub fn get_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn add_callback(&mut self, callback: Box<dyn Fn()>, setting: SettingType) {
        self.callbacks.push((callback, setting));
    }

    pub fn call_callbacks(&self) {
        if !self.enabled {
            return;
        }
        for (callback, enabled) in &self.callbacks {
            if enabled
                .lock()
                .unwrap()
                .borrow()
                .get_value()
                .try_into()
                .unwrap()
            {
                callback();
                log_to_file("called callback");
            }
        }
    }
}
