use std::{sync::Mutex, cell::RefCell, rc::Rc};

use jni::JNIEnv;

use crate::client::{Client, BoxedBingusModule};

pub struct ClientManager {
    client: Mutex<Client>,
    enabled: bool,
}

impl ClientManager {
    pub fn new(jni_env: JNIEnv<'static>) -> Self {
        Self {
            client: Mutex::new(Client::new(jni_env)),
            enabled: false,
        }
    }

    pub fn client_tick(&self) {
        if self.enabled {
            self.client.lock().unwrap().client_tick();
        }
    }

    pub fn get_modules(&self) -> Rc<Vec<Rc<RefCell<BoxedBingusModule>>>> {
        self.client.lock().unwrap().get_modules()
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
}
