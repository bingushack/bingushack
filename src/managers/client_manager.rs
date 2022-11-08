use std::{sync::Mutex, cell::RefCell, rc::Rc, borrow::BorrowMut};

use jni::JNIEnv;

use crate::{client::{BoxedBingusModule, module::{modules::*, BingusModule, SettingType}, MappingsManager}, log_to_file};

pub type ModulesRc = Rc<Vec<Rc<RefCell<BoxedBingusModule>>>>;

pub struct ClientManager {
    tick_callbacks: Vec<(Box<dyn Fn()>, SettingType)>,
    load_callbacks: Vec<Box<dyn Fn()>>,
    jni_env: Rc<JNIEnv<'static>>,
    mappings_manager: Rc<MappingsManager<'static>>,
    modules: ModulesRc,
    enabled: Mutex<RefCell<bool>>,
}

impl ClientManager {
    pub fn new(jni_env: JNIEnv<'static>) -> Self {
        let rc_jni_env = Rc::new(jni_env);
        let mappings_manager = Rc::new(MappingsManager::new(Rc::clone(&rc_jni_env)));
        Self {
            tick_callbacks: Vec::new(),
            load_callbacks: Vec::new(),
            jni_env: Rc::clone(&rc_jni_env),
            mappings_manager: Rc::clone(&mappings_manager),
            enabled: Mutex::new(RefCell::new(false)),
            modules: Rc::new(Vec::new()),
        }
    }

    pub fn init(&mut self, modules: ModulesRc) {
        self.modules = modules;
    }

    pub fn get_modules(&self) -> ModulesRc {
        Rc::clone(&self.modules)
    }
    
    pub fn get_enabled(&self) -> bool {
        *self.enabled.lock().unwrap().borrow()
    }

    pub fn set_enabled(&self, enabled: bool) {
        *(*self.enabled.lock().unwrap()).borrow_mut() = enabled;
    }

    pub fn add_callback(&mut self, callback: Box<dyn Fn()>, setting: SettingType, client_callback_type: ClientCallbackType) {
        match client_callback_type {
            ClientCallbackType::Tick => {
                self.tick_callbacks.push((callback, setting));
            },
            ClientCallbackType::Load => {
                self.load_callbacks.push(callback);
            }
        }
    }

    pub fn call_callbacks(&self, callback_type: ClientCallbackType) {
        match callback_type {
            ClientCallbackType::Tick => {
                if !self.get_enabled() {
                    return;
                }
                for (callback, enabled) in &self.tick_callbacks {
                    if enabled
                        .lock()
                        .unwrap()
                        .borrow()
                        .get_value()
                        .try_into()
                        .unwrap()
                    {
                        callback();
                    }
                }
            },
            ClientCallbackType::Load => {
                for callback in &self.load_callbacks {
                    callback();
                }
            }
        }
    }

    pub fn get_jni_env(&self) -> Rc<JNIEnv<'static>> {
        Rc::clone(&self.jni_env)
    }

    pub fn get_mappings_manager(&self) -> Rc<MappingsManager<'static>> {
        Rc::clone(&self.mappings_manager)
    }
}

pub enum ClientCallbackType {
    Tick,
    Load,
}
