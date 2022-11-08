use std::{sync::{Mutex, Arc}, cell::RefCell, rc::Rc, borrow::BorrowMut};

use jni::JNIEnv;

use crate::{client::{BoxedBingusModule, module::{modules::*, BingusModule, SettingType, Newable, add_client_callback, add_render_method_to_manager}, MappingsManager}, log_to_file};

pub type ModulesRc = Rc<Vec<Rc<RefCell<ModulesEnum>>>>;

pub struct ClientManager {
    tick_callbacks: Vec<(Box<dyn Fn()>, SettingType)>,
    load_callbacks: Vec<Box<dyn Fn()>>,
    jni_env: Rc<JNIEnv<'static>>,
    mappings_manager: Rc<MappingsManager<'static>>,
    modules: RefCell<ModulesRc>,
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
            modules: RefCell::new(Rc::new(Vec::new())),
        }
    }

    pub fn init(&'static self) {
        *self.modules.borrow_mut() = {
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
            // this macro will create and initialize a module to be added to the vector
            macro_rules! modules_initiator {
                ($module_ty:ident) => {{
                    let mut md: ModulesEnum = <$module_ty>::new().into();
                    md.init();
                    let enabled = md.get_enabled_setting();

                    let md_ref = unsafe { std::mem::transmute::<&ModulesEnum, &'static ModulesEnum>(&md) };

                    //md.add_render_method_to_manager().add_client_callback(&self.jni_env, &self.mappings_manager, enabled, ClientCallbackType::Tick)
                    add_render_method_to_manager(md_ref);
                    add_client_callback(md_ref, &self.jni_env, &self.mappings_manager, enabled, ClientCallbackType::Tick);
                    md
                }}
            }


            // in debug mode it needs to be mutable to add the TestModule but otherwise it doesn't need to be
            #[cfg(build = "debug")]
            let mut modules;
            #[cfg(not(build = "debug"))]
            let modules;

            // add all non-debug modules
            modules = modules_maker![
                modules_initiator!(AutoTotem),
                modules_initiator!(Triggerbot),
                modules_initiator!(Esp)
            ];

            // if in debug add debug modules
            #[cfg(build = "debug")]
            modules.append(&mut modules_maker![
                modules_initiator!(TestModule)
            ]);

            Rc::new(modules)
        };;
    }

    pub fn get_modules(&self) -> ModulesRc {
        Rc::clone(&self.modules.borrow())
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

#[derive(Debug, Clone, Copy)]
pub enum ClientCallbackType {
    Tick,
    Load,
}
