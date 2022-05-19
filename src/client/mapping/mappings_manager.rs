use crate::client::mapping::{CM, Mem};
use std::collections::HashMap;

pub struct MappingsManager {
    mappings: HashMap<String, CM>,
}

impl MappingsManager{
    pub fn new() -> Self {
        let mut new_self = Self {
            mappings: HashMap::new(),
        };


        new_self.map_stuff();


        new_self
    }

    fn map_stuff(&mut self) {
        // todo: add obfuscated mappings stuff
        // see https://github.com/UnknownDetectionParty/UDP-CPP/blob/018233f85f81ac0c2f7ccd780844be8a8102d39a/UDP/mapping/Mapping.h#L31

        {
            let class_name = "TitleScreen".to_string();
            let mut title_screen_cm = Self::make(class_name.clone(), "class_442".to_string());
            Self::add_obf_field(
                &mut title_screen_cm,
                "splashText".to_string(),
                "field_2586".to_string(),
                "Ljava/lang/String".to_string(),
                false
            );

            self.mappings.insert(class_name, title_screen_cm);
        }
    }

    pub fn get(&self, class_name: &String) -> Option<&CM> {
        self.mappings.get(class_name)
    }

    pub fn get_hashmap(&self) -> &HashMap<String, CM> {
        &self.mappings
    }


    // todo: move these to CM impl block
    fn make(key: String, name: String) -> CM {
        CM {
            name,
            fields: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    fn add_obf_field(
        cm: &mut CM,
        key_name: String,
        ob_name: String,
        desc: String,
        is_static: bool,
    ) {
        let m = Mem {
            name: ob_name,
            description: desc,
            is_static,
        };
        cm.fields.insert(key_name, m);
    }

    fn add_obf_method(
        cm: &mut CM,
        key_name: String,
        ob_name: String,
        desc: String,
        is_static: bool,
    ) {
        let m = Mem {
            name: ob_name,
            description: desc,
            is_static,
        };
        cm.methods.insert(key_name, m);
    }

    fn add_field(
        cm: &mut CM,
        name: String,
        desc: String,
        is_static: bool,
    ) {
        Self::add_obf_field(cm, name.clone(), name, desc, is_static);
    }

    fn add_method(
        cm: &mut CM,
        name: String,
        desc: String,
        is_static: bool,
    ) {
        Self::add_obf_method(cm, name.clone(), name, desc, is_static);
    }
}
