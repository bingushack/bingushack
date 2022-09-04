use super::{
    AllSettingsType, BingusModule, BingusSettings, BoxedBingusModule, SettingType, SettingValue,
};
use crate::client::{
    mapping::MappingsManager,
    setting::{BooleanSetting, FloatSetting},
};
use crate::{
    apply_object,
    call_method_or_get_field,
};
use jni::{
    objects::JValue,
    JNIEnv,
};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub struct Triggerbot {
    enabled: SettingType,
    settings: AllSettingsType,
}

impl BingusModule for Triggerbot {
    fn new_boxed() -> BoxedBingusModule {
        Box::new(Self {
            enabled: Arc::new(Mutex::new(RefCell::new(BingusSettings::BooleanSetting(
                BooleanSetting::new(SettingValue::from(false), "enabled"),
            )))),
            settings: Arc::new(Mutex::new(RefCell::new(vec![Rc::new(RefCell::new(
                BingusSettings::FloatSetting(FloatSetting::new(
                    SettingValue::from(0.0),
                    "range",
                    0.0..=5.0,
                )),
            ))]))),
        })
    }

    fn tick(&mut self, env: Rc<JNIEnv>, mappings_manager: Rc<MappingsManager>) {
        // check if player is targetting an entity
        let minecraft_client = mappings_manager.get("MinecraftClient").unwrap();
        apply_object!(
            minecraft_client,
            call_method_or_get_field!(env, minecraft_client, "getInstance", true, &[]).unwrap().l().unwrap()
        );

        let player = mappings_manager.get("PlayerEntity").unwrap();
        apply_object!(
            player,
            call_method_or_get_field!(env, minecraft_client, "player", false).unwrap().l().unwrap()
        );

        let target = mappings_manager.get("Optional").unwrap();
        apply_object!(
            target,
            call_method_or_get_field!(
                env,
                mappings_manager.get("DebugRenderer").unwrap(),
                "getTargetedEntity",
                true,
                &[
                    JValue::from(player.get_object().unwrap()),
                    JValue::from(3),  // make it the range setting eventually
                ]
            ).unwrap().l().unwrap()
        );

        if !call_method_or_get_field!(env, target, "isPresent", false, &[]).unwrap().z().unwrap() {
            return;
        }
        let target = {
            let entity = mappings_manager.get("Entity").unwrap();
            apply_object!(
                entity,
                call_method_or_get_field!(env, target, "get", false, &[]).unwrap().l().unwrap()
            );
            entity
        };
        // todo check if correct entity type

        // check if entity is alive
        // check if player is using an item
        if {
            !call_method_or_get_field!(env, target, "isAlive", false, &[]).unwrap().z().unwrap()
            || call_method_or_get_field!(env, player, "isUsingItem", false, &[]).unwrap().z().unwrap()
            || (call_method_or_get_field!(env, player, "getAttackCooldownProgress", false, &[
                call_method_or_get_field!(env, minecraft_client, "getTickDelta", false, &[]).unwrap(),
            ]).unwrap().f().unwrap() != 1.0 )
        } {
            return;
        }

        // attack entity and swing mainhand
        let interaction_manager = mappings_manager.get("InteractionManager").unwrap();
        apply_object!(
            interaction_manager,
            call_method_or_get_field!(env, minecraft_client, "interactionManager", false).unwrap().l().unwrap()
        );

        call_method_or_get_field!(
            env,
            interaction_manager,
            "attackEntity",
            false,
            &[
                JValue::from(player.get_object().unwrap()),
                JValue::from(target.get_object().unwrap()),
            ]
        ).unwrap();
        call_method_or_get_field!(
            env,
            player,
            "swingHand",
            false,
            &[
                call_method_or_get_field!(
                    env,
                    mappings_manager.get("Hand").unwrap(),
                    "MAIN_HAND",
                    true
                ).unwrap(),
                JValue::from(false),
            ]
        ).unwrap();
    }

    fn on_load(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn on_unload(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn on_enable(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn on_disable(&mut self, _env: Rc<JNIEnv>, _mappings_manager: Rc<MappingsManager>) {}

    fn get_all_settings(&self) -> AllSettingsType {
        Arc::clone(&self.settings)
    }

    fn get_enabled_setting(&self) -> SettingType {
        Arc::clone(&self.enabled)
    }

    fn to_name(&self) -> String {
        "Triggerbot".to_string()
    }
}
