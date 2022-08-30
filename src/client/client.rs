use jni::JNIEnv;
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver};
use super::mapping::*;
use crate::ClickGuiMessage;
use super::module::*;
use crate::client::setting::BingusSetting;



pub type RcBoxedBingusSetting = Rc<Box<dyn BingusSetting>>;


pub struct Client {
    rx: Receiver<ClickGuiMessage>,
    tx: Sender<ClickGuiMessage>,

    // prolly a better way to do this with hashmaps/hashsets in the future
    modules: Vec<Box<dyn BingusModule>>,

    env: Rc<JNIEnv<'static>>,
    mappings_manager: Rc<MappingsManager<'static>>,
}


impl Client {
    pub fn new(jni_env: JNIEnv<'static>, rx: Receiver<ClickGuiMessage>, tx: Sender<ClickGuiMessage>) -> Client {
        let env = Rc::new(jni_env);
        Client {
            rx,
            tx,

            modules: vec![
                AutoTotem::new_boxed(),
            ],

            env: env.clone(),

            mappings_manager: Rc::new(MappingsManager::new(env)),
        }
    }

    pub fn client_tick(&mut self) {
        if let Ok(message) = self.rx.try_recv() {
            match message {
                ClickGuiMessage::RunModule(mut module) => module.tick(self.env.clone(), self.mappings_manager.clone()),
                _ => {}
            }
        }
    }
}