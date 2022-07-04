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
use jni::objects::{JClass, JFieldID, JMethodID, JStaticFieldID, JStaticMethodID};

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
impl<'j> Client {
    pub fn new(rx: Receiver<ClickGuiMessage>, tx: Sender<ClickGuiMessage>) -> Self {
        // something in here is broken
        

        let jvm: JavaVM = unsafe {
            use jni::sys::JNI_GetCreatedJavaVMs;

            let jvm_ptr = Vec::with_capacity(1).as_mut_ptr();
            JNI_GetCreatedJavaVMs(jvm_ptr, 1, null_mut());

            JavaVM::from_raw(*jvm_ptr).unwrap()
        };
        jvm.attach_current_thread_as_daemon().unwrap();
        Client {
            rx,
            tx,

            jvm,

            cm_lookup: MappingsManager::new(),
        }
    }

    

    pub fn client_tick(&mut self) {
        if let Ok(message) = self.rx.try_recv() {
            let env = self.jvm.get_env().unwrap();

            match message {
                ClickGuiMessage::Dev(text) => {
                    unsafe {
                        MessageBoxA(
                        null_mut(),
                        CString::new("a1").unwrap().as_ptr(),
                        CString::new("bingushack").unwrap().as_ptr(),
                        MB_OK,
                        );
                    }
                    // set splash screen text to "Hello world!"

                    let fid = self.get_field_id(
                        "TitleScreen".to_string(),
                        "splashText".to_string(),
                    );

                    unsafe {
                        MessageBoxA(
                        null_mut(),
                        CString::new("b1").unwrap().as_ptr(),
                        CString::new("bingushack").unwrap().as_ptr(),
                        MB_OK,
                        );
                    }

                    env.set_field(
                        self.get_class_obj("TitleScreen".to_string()),
                        "splashText".to_string(),
                        "Ljava/lang/String;",
                        JValue::Object(*env.new_string(text).unwrap()),
                    ).unwrap();
                }
                _ => {}
            }
        }
    }



    fn get_env(&'j self) -> JNIEnv<'j> {
        self.jvm.get_env().unwrap()
    }

    fn get_cm_lookup(&self) -> &HashMap<String, CM> {
        &self.cm_lookup.get_hashmap()
    }

    fn get_class_obj(&'j self, class_name: String) -> JClass<'j> {
        let obf_class_name = self.get_class(class_name).get_name();
        self.get_env().find_class(obf_class_name).unwrap()
    }

    fn get_class(&self, key: String) -> &CM {
        self.get_cm_lookup().get(&key).unwrap()
    }

    fn get_class_loader(&'j self) -> JClass<'j> {
        env.find_class("java/lang/ClassLoader").unwrap()
    }

    fn get_field_id(&'j self, class_name: String, name: String) -> BingusJFieldID<'j> {
        let cm = self.get_class(class_name);
        let obf_class_name = cm.get_name();
        let field: &Mem = cm.get_field(&name);
        let env = self.get_env();
        unsafe {
            MessageBoxA(
            null_mut(),
            CString::new("f1").unwrap().as_ptr(),
            CString::new("bingushack").unwrap().as_ptr(),
            MB_OK,
            );
        }  // runs

        unsafe {
            MessageBoxA(
                null_mut(),
                CString::new(format!("obf_class_name:{},field_name:{},field_type:{}", obf_class_name, field.get_name(), field.get_description())).unwrap().as_ptr(),
                CString::new("bingushack").unwrap().as_ptr(),
                MB_OK,
            );
        }
        // something in here breaks
        if field.is_static() {
            BingusJFieldID { static_field: ManuallyDrop::new(
                env.get_static_field_id(obf_class_name, field.get_name(), field.get_description()).unwrap()
            ) }
        } else {
            BingusJFieldID { normal_field: ManuallyDrop::new(
                env.get_field_id(obf_class_name, field.get_name(), field.get_description()).unwrap()
            ) }
        }
    }

    fn get_method_id(&'j self, class_name: String, name: String) -> BingusJMethodID<'j> {
        let cm = self.get_class(class_name);
        let method: &Mem = cm.get_methods().get(&name).unwrap();
        let env = self.get_env();
        if method.is_static() {
            BingusJMethodID { static_method: ManuallyDrop::new(
                env.get_static_method_id(self.get_class(name.clone()).get_name(), name, method.get_description()).unwrap()
            )}
        } else {
            BingusJMethodID { normal_method: ManuallyDrop::new(
                env.get_method_id(self.get_class(name.clone()).get_name(), name, method.get_description()).unwrap()
            ) }
        }
    }
}