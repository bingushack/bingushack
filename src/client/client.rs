use crate::client::module::modules::*;

use super::{mapping::*, module::*};
use jni::JNIEnv;
use std::{
    rc::Rc,
    cell::RefCell,
};

pub type BoxedBingusModule = Box<dyn BingusModule>;

pub struct Client {
    env: Rc<JNIEnv<'static>>,
    mappings_manager: Rc<MappingsManager<'static>>,

    modules: Rc<Vec<Rc<RefCell<BoxedBingusModule>>>>,
}

impl Client {
    pub fn new(
        jni_env: JNIEnv<'static>,
    ) -> Client {
        let env = Rc::new(jni_env);
        Client {
            env: env.clone(),

            mappings_manager: Rc::new(MappingsManager::new(env)),

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

                // add all non-debug modules
                modules = modules_maker![
                    AutoTotem::new_boxed(),
                    Triggerbot::new_boxed(),
                    Esp::new_boxed()
                ];

                // if in debug add debug modules
                #[cfg(build = "debug")]
                modules.extend_from_slice(&modules_maker![
                    TestModule::new_boxed()
                ]);

                Rc::new(modules)
            }
        }
    }

    // called constantly
    pub fn client_tick(&mut self) {
        for module in self.modules.iter() {
            module.borrow_mut().tick(self.env.clone(), self.mappings_manager.clone());
        }
    }

    pub fn get_modules(&self) -> Rc<Vec<Rc<RefCell<BoxedBingusModule>>>> {
        Rc::clone(&self.modules)
    }
}
