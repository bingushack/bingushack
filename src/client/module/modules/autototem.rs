use super::{
    AllSettingsType, BingusModule, BingusSettings, BoxedBingusModule, SettingType,
    SettingValue,
};
use crate::client::setting::RangeSetting;
use crate::{
    apply_object,
    call_method_or_get_field,
};
use crate::client::{mapping::MappingsManager, setting::BooleanSetting};
use jni::{
    objects::JValue,
    JNIEnv,
};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub struct AutoTotem {
    // todo make this enabled settings boilerplate shit a proc macro
    enabled: SettingType,
    settings: AllSettingsType,
    was_enabled: SettingType,

    prev_game_time: i64,
    next_delay: i64,
}

impl BingusModule for AutoTotem {
    fn new_boxed() -> BoxedBingusModule {
        Box::new(Self {
            enabled: Arc::new(Mutex::new(RefCell::new(BingusSettings::BooleanSetting(
                BooleanSetting::new(SettingValue::from(false), "enabled"),
            )))),
            // todo turn this into a hashmap
            settings: Arc::new(Mutex::new(RefCell::new(vec![
                Rc::new(RefCell::new(BingusSettings::RangeSetting(
                    RangeSetting::new(
                        SettingValue::from([160.0, 240.0]),
                        0.0..=240.0,
                        Some(0),
                        Some(1.0),
                        "delay (ticks)"
                    ),
                ))),
            ]))),
            prev_game_time: 0,
            next_delay: 0,
        })
    }

    fn tick(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {
        let minecraft_client = mappings_manager.get("MinecraftClient").unwrap();
        apply_object!(
            minecraft_client,
            call_method_or_get_field!(env, minecraft_client, "getInstance", true, &[]).unwrap().l().unwrap()
        );

        let world = mappings_manager.get("ClientLevel").unwrap();
        apply_object!(
            world,
            call_method_or_get_field!(env, minecraft_client, "level", false).unwrap().l().unwrap()
        );

        let player = mappings_manager.get("PlayerEntity").unwrap();
        apply_object!(
            player,
            call_method_or_get_field!(env, minecraft_client, "player", false).unwrap().l().unwrap()
        );

        let inventory = mappings_manager.get("Inventory").unwrap();
        apply_object!(
            inventory,
            call_method_or_get_field!(env, player, "getInventory", false, &[]).unwrap().l().unwrap()
        );


        let offhand_item = mappings_manager.get("Item").unwrap();
        {
            let item_stack = mappings_manager.get("ItemStack").unwrap();
            apply_object!(
                item_stack,
                call_method_or_get_field!(env, player, "getOffHandStack", false, &[]).unwrap().l().unwrap()
            );
            apply_object!(
                offhand_item,
                call_method_or_get_field!(env, item_stack, "getItem", false, &[]).unwrap().l().unwrap()
            );
        }

        // get TOTEM_OF_UNDYING id
        let totem_of_undying_id = {
            let totem_of_undying = mappings_manager.get("Items").unwrap();
            apply_object!(
                totem_of_undying,
                call_method_or_get_field!(env, totem_of_undying, "TOTEM_OF_UNDYING", true).unwrap().l().unwrap()
            );

            call_method_or_get_field!(
                env,
                offhand_item,
                "getRawId",
                true,
                &[JValue::from(totem_of_undying.get_object().unwrap())]
            )
            .unwrap()
            .i()
            .unwrap()
        };

        let offhand_is_totem = call_method_or_get_field!(
            env,
            offhand_item,
            "getRawId",
            true,
            &[JValue::from(offhand_item.get_object().unwrap())]
        ).unwrap().i().unwrap() == totem_of_undying_id;

        let current_game_time = call_method_or_get_field!(env, world, "getGameTime", false, &[]).unwrap().j().unwrap();
        if !offhand_is_totem {
            // check if the delay is up
            {
                println!("{} + {} > {}", self.prev_game_time, self.next_delay, current_game_time);
                if self.prev_game_time + self.next_delay >= current_game_time {
                    let next_delay = {
                        let settings_mutex_guard = self.settings.lock().unwrap();
                        let settings = settings_mutex_guard.borrow();
                        let range_setting: RangeSetting = settings.get(0).unwrap().borrow().clone().try_into().unwrap();
                        range_setting.get_random_i64_in_range()
                    };
                    self.next_delay = next_delay;
                    self.prev_game_time = current_game_time;
                    return;
                }
            }

            // todo add a check if a totem is even in the inventory with containsAny
            // find totem in inventory
            let mut found_totem_slot: Option<i32> = None;

            // all valid totem slots
            // only works in main inventory, not hotbar for some reason
            for i in 9..45 {
                // potential optimizations GALORE
                if {
                    let i_item_stack = mappings_manager.get("ItemStack").unwrap();
                    // call getStack(i) on inventory then getItem on the result then getRawId on the result of that
                    apply_object!(
                        i_item_stack,
                        call_method_or_get_field!(
                            env,
                            inventory,
                            "getStack",
                            false,
                            &[JValue::from(i)]
                        ).unwrap().l().unwrap()
                    );

                    let i_item = mappings_manager.get("Item").unwrap();
                    apply_object!(
                        i_item,
                        call_method_or_get_field!(env, i_item_stack, "getItem", false, &[]).unwrap().l().unwrap()
                    );

                    call_method_or_get_field!(
                        env,
                        i_item,
                        "getRawId",
                        true,
                        &[JValue::from(i_item.get_object().unwrap())]
                    ).unwrap().i().unwrap() == totem_of_undying_id
                } {
                    found_totem_slot = Some(i);
                    break;
                }
            }

            // swap totem to offhand
            if let Some(found_totem_slot) = found_totem_slot {
                let interaction_manager = mappings_manager.get("InteractionManager").unwrap();
                apply_object!(
                    interaction_manager,
                    call_method_or_get_field!(
                        env,
                        minecraft_client,
                        "interactionManager",
                        false
                    ).unwrap().l().unwrap()
                );

                let current_screen_handler = mappings_manager.get("ScreenHandler").unwrap();
                apply_object!(
                    current_screen_handler,
                    call_method_or_get_field!(
                        env,
                        player,
                        "currentScreenHandler",
                        false
                    ).unwrap().l().unwrap()
                );

                let sync_id = call_method_or_get_field!(
                    env,
                    current_screen_handler,
                    "syncId",
                    false
                ).unwrap().i().unwrap();

                let pickup_slot_action = call_method_or_get_field!(
                    env,
                    mappings_manager.get("SlotActionType").unwrap(),
                    "PICKUP",
                    true
                ).unwrap();

                // call clickSlot
                // pick up
                call_method_or_get_field!(
                    env,
                    interaction_manager,
                    "clickSlot",
                    false,
                    &[
                        JValue::from(sync_id),
                        JValue::from(found_totem_slot),
                        JValue::from(0),
                        pickup_slot_action,
                        JValue::from(player.get_object().unwrap()),
                    ]
                ).unwrap();

                // put down
                call_method_or_get_field!(
                    env,
                    interaction_manager,
                    "clickSlot",
                    false,
                    &[
                        JValue::from(sync_id),
                        JValue::from(45),  // 45 is offhand slot index
                        JValue::from(0),
                        pickup_slot_action,
                        JValue::from(player.get_object().unwrap()),
                    ]
                ).unwrap();
            }
        }
    }

    fn on_load(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn on_unload(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn on_enable(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {
        let minecraft_client = mappings_manager.get("MinecraftClient").unwrap();
        apply_object!(
            minecraft_client,
            call_method_or_get_field!(env, minecraft_client, "getInstance", true, &[]).unwrap().l().unwrap()
        );

        let world = mappings_manager.get("ClientLevel").unwrap();
        apply_object!(
            world,
            call_method_or_get_field!(env, minecraft_client, "level", false).unwrap().l().unwrap()
        );

        let current_game_time = call_method_or_get_field!(env, world, "getGameTime", false, &[]).unwrap().j().unwrap();

        self.prev_game_time = current_game_time;
    }

    fn on_disable(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn get_all_settings(&self) -> AllSettingsType {
        Arc::clone(&self.settings)
    }

    fn get_enabled_setting(&self) -> SettingType {
        Arc::clone(&self.enabled)
    }

    fn get_was_enabled_setting(&self) -> SettingType {
        Arc::clone(&self.was_enabled)
    }

    fn to_name(&self) -> String {
        "AutoTotem".to_string()
    }
}
