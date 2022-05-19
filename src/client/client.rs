use jni::{
    JNIEnv, 
    JavaVM,
    sys::{
        JNI_GetCreatedJavaVMs,
        jint,
    },
};
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use super::mapping::MappingsManager;
use crate::ClickGuiMessage;


pub struct Client {
    rx: Receiver<ClickGuiMessage>,
    tx: Sender<ClickGuiMessage>,

    jvm: JavaVM,

    cm_lookup: MappingsManager,
    // todo: make Minecraft struct and add as a field here
}

impl Client {
    pub fn new(rx: Receiver<ClickGuiMessage>, tx: Sender<ClickGuiMessage>) -> Self {
        // something in here is broken
        use jni::sys::JNI_GetCreatedJavaVMs;

        let jvm = std::ptr::null_mut();
        let jvm: JavaVM = unsafe {
            JNI_GetCreatedJavaVMs(jvm, 1, std::ptr::null_mut());
            let jvm = JavaVM::from_raw(*jvm).unwrap();
            jvm
        };
        jvm.attach_current_thread_as_daemon().unwrap();
        Client {
            rx,
            tx,

            jvm,

            cm_lookup: MappingsManager::new(),
        }
    }

    pub fn get_env(&self) -> JNIEnv {
        self.jvm.get_env().unwrap()
    }

    pub fn client_tick(&mut self) {
        if let Ok(message) = self.rx.try_recv() {
            match message {
                ClickGuiMessage::Dev(text) => {
                    let title_screen_cm = self.cm_lookup.get(&"TitleScreen".to_string()).unwrap();
                    let splash_text_field = title_screen_cm.get_field(&"splashText".to_string());
                    let (name, description) = (splash_text_field.get_name(), splash_text_field.get_description());
                    self.tx.send(ClickGuiMessage::Dev(format!("{} {}", name, description)));
                }
                _ => {}
            }
        }
    }

    pub fn get_cm_lookup(&self) -> &MappingsManager {
        &self.cm_lookup
    }
}
