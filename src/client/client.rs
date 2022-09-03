use jni::JNIEnv;
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver};
use super::mapping::*;
use crate::ClickGuiMessage;
use super::module::*;
use crate::client::setting::BingusSetting;



pub type BoxedBingusModule = Box<dyn BingusModule>;


pub struct Client {
    rx: Receiver<ClickGuiMessage>,
    tx: Sender<ClickGuiMessage>,

    env: Rc<JNIEnv<'static>>,
    mappings_manager: Rc<MappingsManager<'static>>,
}


impl Client {
    pub fn new(jni_env: JNIEnv<'static>, rx: Receiver<ClickGuiMessage>, tx: Sender<ClickGuiMessage>) -> Client {
        let env = Rc::new(jni_env);
        Client {
            rx,
            tx,

            env: env.clone(),

            mappings_manager: Rc::new(MappingsManager::new(env)),
        }
    }

    pub fn client_tick(&mut self) {
        if let Ok(message) = self.rx.try_recv() {
            match message {
                ClickGuiMessage::RunModule(module) => {
                    module.borrow_mut().tick(self.env.clone(), self.mappings_manager.clone());
                },
            }
        }
    }
}