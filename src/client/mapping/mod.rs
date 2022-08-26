mod mappings_manager;

pub use self::mappings_manager::MappingsManager;

use jni::objects::{JObject, JClass};
use jni::JNIEnv;

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CM<'j> {
    class: Option<JClass<'j>>,  // todo try to make this not an option
    object: RefCell<Option<JObject<'j>>>,  // bruh

    fields: HashMap<String, Mem>,
    static_fields: HashMap<String, StaticMem>,

    methods: HashMap<String, Mem>,
    static_methods: HashMap<String, StaticMem>,
}

impl<'j> CM<'j> {
    pub fn get_field(&self, name: &str) -> Option<&Mem> {
        self.fields.get(&name.to_string())
    }

    pub fn get_static_field(&self, name: &str) -> Option<&StaticMem> {
        self.static_fields.get(&name.to_string())
    }

    pub fn get_method(&self, name: &str) -> Option<&Mem> {
        self.methods.get(&name.to_string())
    }

    pub fn get_static_method(&self, name: &str) -> Option<&StaticMem> {
        self.static_methods.get(&name.to_string())
    }

    pub fn get_class(&self) -> JClass<'j> {
        self.class.unwrap()
    }

    pub fn get_object(&self) -> Option<JObject<'j>> {
        *self.object.borrow()
    }

    
    // makes object not None
    pub fn apply_object(&self, object: JObject<'j>) {
        *self.object.borrow_mut() = Some(object);
    }

    pub fn apply_class(&mut self, class: JClass<'j>) {
        self.class = Some(class);
    }
}



pub trait MemTrait {
    fn get_name(&self) -> String;

    fn get_sig(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct Mem {
    name: String,
    sig: String,
}


impl MemTrait for Mem {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_sig(&self) -> String {
        self.sig.clone()
    }
}

#[derive(Clone, Debug)]
pub struct StaticMem {
    mem: Mem
}

impl MemTrait for StaticMem {
    fn get_name(&self) -> String {
        self.mem.name.clone()
    }

    fn get_sig(&self) -> String {
        self.mem.sig.clone()
    }
}
