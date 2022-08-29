mod mappings_manager;

pub use self::mappings_manager::MappingsManager;

use jni::objects::{JObject, JClass};

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CM<'j> {
    class: RefCell<Option<JClass<'j>>>,
    object: RefCell<Option<JObject<'j>>>,  // bruh

    fields: HashMap<String, Mem>,
    static_fields: HashMap<String, StaticMem>,

    methods: HashMap<String, Mem>,
    static_methods: HashMap<String, StaticMem>,
}

impl<'j> Default for CM<'j> {
    fn default() -> CM<'j> {
        CM {
            class: RefCell::new(None),
            object: RefCell::new(None),
    
            fields: HashMap::new(),
            static_fields: HashMap::new(),
    
            methods: HashMap::new(),
            static_methods: HashMap::new()
        }
    }
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
        (*self.class.borrow()).unwrap()
    }

    pub fn get_object(&self) -> Option<JObject<'j>> {
        *self.object.borrow()
    }

    
    // makes object not None
    pub fn apply_object(&self, object: JObject<'j>) {
        *self.object.borrow_mut() = Some(object);
    }

    pub fn apply_class(&self, class: JClass<'j>) {
        *self.class.borrow_mut() = Some(class);
    }






    fn add_method(
        &mut self,
        key_name: String,
        ob_name: String,
        sig: String,
        is_static: bool,
    ) {
        let m = Mem {
            name: ob_name,
            sig,
        };
    
        if is_static {
            self.static_methods.insert(key_name, StaticMem { mem: m } );
        } else {
            self.methods.insert(key_name, m);
        }
    }

    fn add_field(
        &mut self,
        key_name: String,
        ob_name: String,
        sig: String,
        is_static: bool,
    ) {
        let m = Mem {
            name: ob_name,
            sig,
        };
    
        if is_static {
            self.static_fields.insert(key_name, StaticMem { mem: m } );
        } else {
            self.fields.insert(key_name, m);
        }
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
