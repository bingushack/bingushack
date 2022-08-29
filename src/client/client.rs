use jni::JNIEnv;
use std::sync::mpsc::{Sender, Receiver};
use super::{
    mapping::*,
    modules_enum::Modules,
};
use crate::ClickGuiMessage;
use jni::objects::{JValue, JObject};




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
                        Modules::AutoTotem(_, _) => {
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
                            let item_stack = self.mappings.get("ItemStack").unwrap();
                            {
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

                            // if offhand is not a totem
                            if {
                                // compare to the offhand item
                                let get_raw_id_method = offhand_item.get_static_method("getRawId").unwrap();
                                let offhand_item_id = self.env.call_static_method(
                                    offhand_item.get_class(),
                                    get_raw_id_method.get_name(),
                                    get_raw_id_method.get_sig(),
                                    &[JValue::from(offhand_item.get_object().unwrap())]
                                ).unwrap().i().unwrap();
        
                                offhand_item_id != totem_of_undying_id
                            }
                            {
                                // todo add a check if a totem is even in the inventory with containsAny
                                // find totem in inventory
                                let mut found_totem_slot: Option<i32> = None;

                                let get_stack_method = inventory.get_method("getStack").unwrap();
                                let get_item_method = item_stack.get_method("getItem").unwrap();
                                let get_raw_id_method = offhand_item.get_static_method("getRawId").unwrap();
                                // all valid totem slots
                                // only works in main inventory, not hotbar for some reason
                                for i in 9..45 {
                                    if {
                                        let i_item_stack = self.mappings.get("ItemStack").unwrap();
                                        let i_item = self.mappings.get("Item").unwrap();
                                        // call getStack(i) on inventory then getItem on the result then getRawId on the result of that
                                        let stack_object: JObject<'_> = self.env.call_method(
                                            inventory.get_object().unwrap(),
                                            get_stack_method.get_name(),
                                            get_stack_method.get_sig(),
                                            &[JValue::from(i)]
                                        ).unwrap().l().unwrap();
                                        i_item_stack.apply_object(stack_object);

                                        let item_object: JObject<'_> = self.env.call_method(
                                            i_item_stack.get_object().unwrap(),
                                            get_item_method.get_name(),
                                            get_item_method.get_sig(),
                                            &[]
                                        ).unwrap().l().unwrap();
                                        i_item.apply_object(item_object);

                                        self.env.call_static_method(
                                            i_item.get_class(),
                                            get_raw_id_method.get_name(),
                                            get_raw_id_method.get_sig(),
                                            &[JValue::from(i_item.get_object().unwrap())]
                                        ).unwrap().i().unwrap() == totem_of_undying_id
                                    } {
                                        found_totem_slot = Some(i);
                                        break;
                                    }
                                }


                                // swap totem to offhand
                                if let Some(found_totem_slot) = found_totem_slot {
                                    let interaction_manager = self.mappings.get("InteractionManager").unwrap();
                                    let interaction_manager_mappings = minecraft_client.get_field("interactionManager").unwrap();
                                    let interaction_manager_object = self.env.get_field(
                                        minecraft_client.get_object().unwrap(),
                                        interaction_manager_mappings.get_name(),
                                        interaction_manager_mappings.get_sig(),
                                    ).unwrap().l().unwrap();
                                    interaction_manager.apply_object(interaction_manager_object);

                                    let current_screen_handler = self.mappings.get("ScreenHandler").unwrap();
                                    let current_screen_handler_mappings = player.get_field("currentScreenHandler").unwrap();
                                    let current_screen_handler_object = self.env.get_field(
                                        player.get_object().unwrap(),
                                        current_screen_handler_mappings.get_name(),
                                        current_screen_handler_mappings.get_sig(),
                                    ).unwrap().l().unwrap();
                                    current_screen_handler.apply_object(current_screen_handler_object);

                                    let sync_id_mappings = current_screen_handler.get_field("syncId").unwrap();
                                    let sync_id = self.env.get_field(
                                        current_screen_handler.get_object().unwrap(),
                                        sync_id_mappings.get_name(),
                                        sync_id_mappings.get_sig(),
                                    ).unwrap().i().unwrap();


                                    let slot_action = self.mappings.get("SlotActionType").unwrap();
                                    let pickup_slot_mappings = slot_action.get_static_field("PICKUP").unwrap();
                                    let pickup_slot_action = self.env.get_static_field(
                                        slot_action.get_class(),
                                        pickup_slot_mappings.get_name(),
                                        pickup_slot_mappings.get_sig(),
                                    ).unwrap();

                                    // call clickSlot
                                    let click_slot_method = interaction_manager.get_method("clickSlot").unwrap();
                                    // pick up
                                    self.env.call_method(
                                        interaction_manager.get_object().unwrap(),
                                        click_slot_method.get_name(),
                                        click_slot_method.get_sig(),
                                        &[
                                            JValue::from(sync_id),
                                            JValue::from(found_totem_slot),
                                            JValue::from(0),
                                            pickup_slot_action,
                                            JValue::from(player.get_object().unwrap()),
                                        ],
                                    ).unwrap();
                                    // put down
                                    self.env.call_method(
                                        interaction_manager.get_object().unwrap(),
                                        click_slot_method.get_name(),
                                        click_slot_method.get_sig(),
                                        &[
                                            JValue::from(sync_id),
                                            JValue::from(45),
                                            JValue::from(0),
                                            pickup_slot_action,
                                            JValue::from(player.get_object().unwrap()),
                                        ],
                                    ).unwrap();
                                }
                            }
                        },
                    }
                }
                _ => {}
            }
        }
    }
}