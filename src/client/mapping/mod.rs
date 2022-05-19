mod minecraft;
mod mappings_manager;

use super::Client;
pub use self::mappings_manager::MappingsManager;
use jni::objects::{JClass, JFieldID, JMethodID, JStaticFieldID, JStaticMethodID};


#[repr(C)]
pub union BingusJFieldID<'a> {
    pub normal_field: JFieldID<'a>,
    pub static_field: JStaticFieldID<'a>,
}

#[repr(C)]
pub union BingusJMethodID<'a> {
    pub normal_method: JMethodID<'a>,
    pub static_method: JStaticMethodID<'a>,
}

pub trait GetClass<'a> {
    fn get_client(&self) -> &Client;

    fn get_class_mut(&'a mut self) -> Option<&mut JClass<'a>>;  // might need a lifetime indicator

    fn get_cm_lookup(&self) -> &HashMap<String, CM>;

    fn get_class_key(&self) -> &str;

    fn get_class(&self, key: String) -> &CM {
        self.get_cm_lookup().get(&key).unwrap()
    }

    fn get_field_id(&self, name: String) -> BingusJFieldID<'a> {
        let cm = self.get_class(self.get_class_key().to_string());
        let field: &Mem = cm.get_fields().get(&name).unwrap();
        let env = self.get_client().get_env();
        if field.is_static() {
            BingusJFieldID { static_field: env.get_static_field_id(self.get_class(name.clone()).get_name(), name, field.get_description()).unwrap() }
        } else {
            BingusJFieldID { normal_field: env.get_field_id(self.get_class(name.clone()).get_name(), name, field.get_description()).unwrap() }
        }
    }

    fn get_method_id(&self, name: String) -> BingusJMethodID<'a> {
        let cm: &'a CM = self.get_class(self.get_class_key().to_string());
        let method: &Mem = cm.get_methods().get(&name).unwrap();
        let env = self.get_client().get_env();
        if method.is_static() {
            BingusJMethodID { static_method: env.get_static_method_id(self.get_class(name.clone()).get_name(), name, method.get_description()).unwrap() }
        } else {
            BingusJMethodID { normal_method: env.get_method_id(self.get_class(name.clone()).get_name(), name, method.get_description()).unwrap() }
        }
    }
}



use std::collections::HashMap;

pub struct CM {
    name: String,
    fields: HashMap<String, Mem>,
    methods: HashMap<String, Mem>,
}

impl CM {
    pub fn get_fields(&self) -> &HashMap<String, Mem> {
        &self.fields
    }

    pub fn get_field(&self, name: &String) -> &Mem {
        self.fields.get(name).unwrap()
    }

    pub fn get_methods(&self) -> &HashMap<String, Mem> {
        &self.methods
    }

    pub fn get_method(&self, name: &String) -> &Mem {
        self.methods.get(name).unwrap()
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

pub struct Mem {
    name: String,
    description: String,
    is_static: bool,
}

impl Mem {
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
