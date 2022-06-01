mod mappings_manager;

pub use self::mappings_manager::MappingsManager;


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

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
