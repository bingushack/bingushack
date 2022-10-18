use super::{mapping::*, module::*};
use crate::ClickGuiMessage;
use jni::JNIEnv;
use std::{
    rc::Rc,
    sync::mpsc::{Receiver, Sender},
};

pub type BoxedBingusModule = Box<dyn BingusModule>;

pub struct Client {
    rx: Receiver<ClickGuiMessage>,
    #[allow(dead_code)]
    tx: Sender<ClickGuiMessage>,  // not used yet

    env: Rc<JNIEnv<'static>>,
    mappings_manager: Rc<MappingsManager<'static>>,
}

impl Client {
    pub fn new(
        jni_env: JNIEnv<'static>,
        rx: Receiver<ClickGuiMessage>,
        tx: Sender<ClickGuiMessage>,
    ) -> Client {
        let env = Rc::new(jni_env);
        Client {
            rx,
            tx,

            env: env.clone(),

            mappings_manager: Rc::new(MappingsManager::new(env)),
        }
    }

    // called constantly
    pub fn client_tick(&mut self) {
        if let Ok(message) = self.rx.try_recv() {
            match message {
                ClickGuiMessage::RunModule(module) => {
                    module
                        .borrow_mut()
                        .tick(self.env.clone(), self.mappings_manager.clone());
                },
                _ => {},
            }
        }
    }
}
