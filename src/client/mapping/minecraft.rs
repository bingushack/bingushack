use jni::objects::{JObject, JClass};
use std::collections::HashMap;
use super::CM;
use crate::client::Client;


pub struct Minecraft<'a> {
    client: Client,
    class: JClass<'a>,
    method_get_minecraft: Option<JObject<'a>>,
    field_player: Option<JObject<'a>>,
    field_world: Option<JObject<'a>>,
    class_key: String,
}

impl<'a, 'b: 'a> super::GetClass<'b> for Minecraft<'a> {
    fn get_client(&self) -> &Client {
        &self.client
    }

    fn get_class_mut(&'b mut self) -> Option<&mut JClass<'b>> {  // might need a lifetime indicator
        Some(&mut self.class)
    }

    fn get_class_key(&self) -> &str {
        &self.class_key
    }

    fn get_cm_lookup(&self) -> &HashMap<String, CM> {
        &self.client.get_cm_lookup().get_hashmap()
    }
}

impl<'a> Minecraft<'a> {
    pub fn new(client: Client, class_name: &str) -> Self {
        let class = client.get_env().find_class(class_name).unwrap();
        Self {
            client,
            class,
            method_get_minecraft: None,
            field_player: None,
            field_world: None,
            class_key: class_name.to_string(),
        }
    }

    // todo: load the hooks with a method (so they aren't called prematurely and crash the whole circus)
}
