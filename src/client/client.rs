#[allow(non_snake_case)]

use jni::{
    JNIEnv, 
    JavaVM,
    sys::{
        JNI_GetCreatedJavaVMs,
        jint,
    },
    objects::JValue,
};
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use super::mapping::MappingsManager;
use crate::ClickGuiMessage;
use crate::client::mapping::*;
use std::ffi::CString;
use std::mem::ManuallyDrop;
use jni::objects::{JString, JObject, JClass, JFieldID, JMethodID, JStaticFieldID, JStaticMethodID};
use jni::signature::JavaType;

use crate::{
    MB_OK,
    null_mut,
    MessageBoxA,
};


#[repr(C)]
pub union BingusJFieldID<'a> {
    pub normal_field: ManuallyDrop<JFieldID<'a>>,
    pub static_field: ManuallyDrop<JStaticFieldID<'a>>,
}

#[repr(C)]
pub union BingusJMethodID<'a> {
    pub normal_method: ManuallyDrop<JMethodID<'a>>,
    pub static_method: ManuallyDrop<JStaticMethodID<'a>>,
}

impl Drop for BingusJFieldID<'_> {
    fn drop(&mut self) {
        unsafe {
            match self {
                BingusJFieldID { normal_field: f } => {
                    ManuallyDrop::drop(f);
                },
                BingusJFieldID { static_field: f } => {
                    ManuallyDrop::drop(f);
                },
            }
        }
    }
}

impl Drop for BingusJMethodID<'_> {
    fn drop(&mut self) {
        unsafe {
            match self {
                BingusJMethodID { normal_method: m } => {
                    ManuallyDrop::drop(m);
                },
                BingusJMethodID { static_method: m } => {
                    ManuallyDrop::drop(m);
                },
            }
        }
    }
}


pub struct Client {
    rx: Receiver<ClickGuiMessage>,
    tx: Sender<ClickGuiMessage>,

    jvm: JavaVM,

    cm_lookup: MappingsManager,
}


// todo clean up all the get_env() calls
impl Client {
    pub fn new(rx: Receiver<ClickGuiMessage>, tx: Sender<ClickGuiMessage>) -> Self {
        let java_vm: JavaVM = unsafe {
            use jni::sys::JNI_GetCreatedJavaVMs;

            let jvm_ptr = Vec::with_capacity(1).as_mut_ptr();
            JNI_GetCreatedJavaVMs(jvm_ptr, 1, null_mut());

            JavaVM::from_raw(*jvm_ptr).unwrap()
        };
        java_vm.attach_current_thread_as_daemon().unwrap();
        Client {
            rx,
            tx,

            jvm: java_vm,

            cm_lookup: MappingsManager::new(),
        }
    }

    fn get_jni_env(&self) -> JNIEnv<'_> {
        self.jvm.get_env().unwrap()
    }

    pub fn client_tick(&mut self) {
        if let Ok(message) = self.rx.try_recv() {
            match message {
                ClickGuiMessage::Dev(text) => {
                    let env = self.get_jni_env();

                    let minecraft_client_class: JClass<'_> = env.find_class("dyr").unwrap();

                    let minecraft_client_object: JObject<'_> = env
                        .call_static_method(minecraft_client_class, "D", "()Ldyr;", &[])
                        .unwrap()
                        .l()
                        .unwrap();


                    let player = env
                        .get_field(minecraft_client_object, "s", "Lepw;")
                        .unwrap()
                        .l()
                        .unwrap();


                    // send `text` in chat
                    env.call_method(player, "e", "(Ljava/lang/String;)V", &[JValue::from(env.new_string(text).unwrap())]);
                }
                _ => {}
            }
        }
    }


    fn get_cm_lookup(&self) -> &HashMap<String, CM> {
        &self.cm_lookup.get_hashmap()
    }

    fn get_class(&self, key: String) -> &CM {
        self.get_cm_lookup().get(&key).unwrap()
    }
}