use jni::JNIEnv;
use std::sync::mpsc::{Sender, Receiver};
use super::{
    mapping::*,
    modules_enum::Modules,
};
use crate::ClickGuiMessage;
use jni::objects::{JValue, JObject};
use crate::message_box;




pub struct Client<'j> {
    rx: Receiver<ClickGuiMessage>,
    tx: Sender<ClickGuiMessage>,

    env: JNIEnv<'j>,

    mappings: MappingsManager<'j>,
}


impl<'j> Client<'j> {
    pub fn new(jni_env: JNIEnv<'j>, rx: Receiver<ClickGuiMessage>, tx: Sender<ClickGuiMessage>) -> Client<'j> {
        Client {
            rx,
            tx,

            env: jni_env,

            mappings: MappingsManager::new(jni_env),
        }
    }

    pub fn client_tick(&mut self) {
        if let Ok(message) = self.rx.try_recv() {
            match message {
                ClickGuiMessage::RunModule(module) => {
                    match module {
                        Modules::AutoTotem => {
                            let minecraft_client = self.mappings.get("MinecraftClient").unwrap();
                            {
                                let get_instance_method = minecraft_client.get_static_method("getInstance").unwrap();
                                let minecraft_client_object: JObject<'_> = self.env.call_static_method(
                                    minecraft_client.get_class(),
                                    get_instance_method.get_name(),
                                    get_instance_method.get_sig(),
                                    &[]
                                ).unwrap().l().unwrap();
                                minecraft_client.apply_object(minecraft_client_object);
                            }
        
                            let player = self.mappings.get("PlayerEntity").unwrap();
                            {
                                let player_mappings = minecraft_client.get_field("player").unwrap();
                                let player_object: JObject<'_> = self.env.get_field(
                                    minecraft_client.get_object().unwrap(),
                                    player_mappings.get_name(),
                                    player_mappings.get_sig(),
                                ).unwrap().l().unwrap();
                                player.apply_object(player_object);
                            }
        
                            let inventory = self.mappings.get("Inventory").unwrap();
                            {
                                let get_inventory_method = player.get_method("getInventory").unwrap();
                                let inventory_object: JObject<'_> = self.env.call_method(
                                    player.get_object().unwrap(),
                                    get_inventory_method.get_name(),
                                    get_inventory_method.get_sig(),
                                    &[]
                                ).unwrap().l().unwrap();
                                inventory.apply_object(inventory_object);
                            }
        
                            let offhand_item = self.mappings.get("Item").unwrap();
                            {
                                let item_stack = self.mappings.get("ItemStack").unwrap();
                                
                                let offhand_stack_method = player.get_method("getOffHandStack").unwrap();
                                let offhand_stack_object: JObject<'_> = self.env.call_method(
                                    player.get_object().unwrap(),
                                    offhand_stack_method.get_name(),
                                    offhand_stack_method.get_sig(),
                                    &[]
                                ).unwrap().l().unwrap();
                                item_stack.apply_object(offhand_stack_object);
        
                                // man i need macros
                                let get_item_method = item_stack.get_method("getItem").unwrap();
                                let offhand_item_object: JObject<'_> = self.env.call_method(
                                    item_stack.get_object().unwrap(),
                                    get_item_method.get_name(),
                                    get_item_method.get_sig(),
                                    &[]
                                ).unwrap().l().unwrap();
                                offhand_item.apply_object(offhand_item_object);
                            }
        
        
        
                            // if offhand is not a totem
                            if !{
                                // get TOTEM_OF_UNDYING id
                                let totem_of_undying_id = {
                                    let totem_of_undying = self.mappings.get("Items").unwrap();
                                    let totem_of_undying_mappings = totem_of_undying.get_static_field("TOTEM_OF_UNDYING").unwrap();
                                    let totem_of_undying_object = self.env.get_static_field(
                                        totem_of_undying.get_class(),
                                        totem_of_undying_mappings.get_name(),
                                        totem_of_undying_mappings.get_sig(),
                                    ).unwrap().l().unwrap();
                                    totem_of_undying.apply_object(totem_of_undying_object);
        
                                    let get_raw_id_method = offhand_item.get_static_method("getRawId").unwrap();
        
                                    self.env.call_static_method(
                                        offhand_item.get_class(),
                                        get_raw_id_method.get_name(),
                                        get_raw_id_method.get_sig(),
                                        &[JValue::from(totem_of_undying.get_object().unwrap())]
                                    ).unwrap().i().unwrap()
                                };
        
                                // compare to the offhand item
                                let get_raw_id_method = offhand_item.get_static_method("getRawId").unwrap();
                                let offhand_item_id = self.env.call_static_method(
                                    offhand_item.get_class(),
                                    get_raw_id_method.get_name(),
                                    get_raw_id_method.get_sig(),
                                    &[JValue::from(offhand_item.get_object().unwrap())]
                                ).unwrap().i().unwrap();
        
                                offhand_item_id == totem_of_undying_id
                            }
                            {
                                message_box("no totem");
                            }
                        },
                        _ => unimplemented!(),
                    }
                }
                _ => {}
            }
        }
    }
}