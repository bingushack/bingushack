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

impl<'j> Client {
    pub fn new(rx: Receiver<ClickGuiMessage>, tx: Sender<ClickGuiMessage>) -> Self {
        // something in here is broken
        

        let jvm: JavaVM = unsafe {
            use winapi::um::libloaderapi::{GetProcAddress, GetModuleHandleA};
            use jni::sys::JNI_GetCreatedJavaVMs;

            /*let jvm = GetProcAddress(
                GetModuleHandleA(CString::new("jvm.dll").unwrap().as_ptr()),
                CString::new("JNI_GetCreatedJavaVMs").unwrap().as_ptr(),
            ) as *mut _;*/

            MessageBoxA(
                null_mut(),
                CString::new("-3").unwrap().as_ptr(),
                CString::new("bingushack").unwrap().as_ptr(),
                MB_OK,
            );

            let jvm_ptr = null_mut();

            JNI_GetCreatedJavaVMs(jvm_ptr, 1, null_mut());

            MessageBoxA(
                null_mut(),
                CString::new("-2").unwrap().as_ptr(),
                CString::new("bingushack").unwrap().as_ptr(),
                MB_OK,
            );

            let jvm = JavaVM::from_raw(*jvm_ptr).unwrap();

            MessageBoxA(
                null_mut(),
                CString::new("-1").unwrap().as_ptr(),
                CString::new("bingushack").unwrap().as_ptr(),
                MB_OK,
            );

            jvm
        };
        unsafe {
            MessageBoxA(
                null_mut(),
                CString::new("0").unwrap().as_ptr(),
                CString::new("bingushack").unwrap().as_ptr(),
                MB_OK,
            );
        }
        jvm.attach_current_thread_as_daemon().unwrap();
        unsafe {
            MessageBoxA(
                null_mut(),
                CString::new("1").unwrap().as_ptr(),
                CString::new("bingushack").unwrap().as_ptr(),
                MB_OK,
            );
        }
        Client {
            rx,
            tx,

            jvm,

            cm_lookup: MappingsManager::new(),
        }
    }

    

    pub fn client_tick(&mut self) {
        if let Ok(message) = self.rx.try_recv() {
            unsafe {
                MessageBoxA(
                null_mut(),
                CString::new("a").unwrap().as_ptr(),
                CString::new("bingushack").unwrap().as_ptr(),
                MB_OK,
                );
            }

            let env = self.jvm.get_env().unwrap();

            unsafe {
                MessageBoxA(
                null_mut(),
                CString::new("b").unwrap().as_ptr(),
                CString::new("bingushack").unwrap().as_ptr(),
                MB_OK,
                );
            }

            match message {
                ClickGuiMessage::Dev(text) => {
                    // set splash screen text to "Hello world!"
                    // TitleScreen is in the package net.minecraft.client.gui.screen.TitleScreen
                    //let title_screen_class = env.find_class().unwrap();
                
                    let clazz = env.find_class("net/minecraft/client/gui/screen/TitleScreen").unwrap();
                    let fid = self.get_field_id(
                        "net/minecraft/client/gui/screen/TitleScreen".to_string(),
                        "splashText".to_string(),
                    );

                    env.set_field(
                        clazz,
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

    fn get_class(&self, key: String) -> &CM {
        self.get_cm_lookup().get(&key).unwrap()
    }

    fn get_field_id(&'j self, class_name: String, name: String) -> BingusJFieldID<'j> {
        let cm = self.get_class(class_name);
        let field: &Mem = cm.get_fields().get(&name).unwrap();
        let env = self.get_env();
        if field.is_static() {
            BingusJFieldID { static_field: ManuallyDrop::new(
                env.get_static_field_id(self.get_class(name.clone()).get_name(), name, field.get_description()).unwrap()
            ) }
        } else {
            BingusJFieldID { normal_field: ManuallyDrop::new(
                env.get_field_id(self.get_class(name.clone()).get_name(), name, field.get_description()).unwrap()
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