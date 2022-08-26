use jni::{
    JNIEnv, 
    JavaVM,
    sys::{
        JNI_GetCreatedJavaVMs,
        jint,
    },
    objects::JValue,
};
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use super::mapping::*;
use crate::ClickGuiMessage;
use crate::client::mapping::*;
use std::ffi::CString;
use std::mem::ManuallyDrop;
use jni::objects::{JString, JObject, JClass, JList};
use jni::signature::JavaType;
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
                ClickGuiMessage::Dev(text) => {
                    let mut minecraft_client = self.mappings.get("MinecraftClient").unwrap();
                    {
                        let get_instance_method = minecraft_client.get_static_method("getInstance").unwrap();
                        let minecraft_client_object: JObject<'_> = self.env
                            .call_static_method(minecraft_client.get_class(), get_instance_method.get_name(), get_instance_method.get_sig(), &[])
                            .unwrap()
                            .l()
                            .unwrap();
                        minecraft_client.apply_object(minecraft_client_object);
                    }
                    let mut world = self.mappings.get("ClientLevel").unwrap();
                    {
                        let level_mappings = minecraft_client.get_field("level").unwrap();
                        let level_field_object = self.env
                            .get_field(minecraft_client.get_object().unwrap(), level_mappings.get_name(), level_mappings.get_sig())
                            .unwrap()
                            .l()
                            .unwrap();
                        world.apply_object(level_field_object);
                    }
                    let mut players_list = self.mappings.get("List").unwrap();  // of type `java/util/List`
                    {
                        let get_players_method = world.get_method("players").unwrap();
                        let players_list_object = self.env
                            .call_method(world.get_object().unwrap(), get_players_method.get_name(), get_players_method.get_sig(), &[])
                            .unwrap()
                            .l()
                            .unwrap();
                        players_list.apply_object(players_list_object);
                    }


                    let players_list: JList = self.env.get_list(players_list.get_object().unwrap()).unwrap();

                    for i in 0..players_list.size().unwrap() {
                        let player_living_entity = self.mappings.get("LivingEntity").unwrap();
                        player_living_entity.apply_object(players_list.get(i).unwrap().unwrap());

                        // apply glowing
                        let force_add_effects_method = player_living_entity.get_method("forceAddEffect").unwrap();
                        self.env.
                            call_method(
                                player_living_entity.get_object().unwrap(),
                                force_add_effects_method.get_name(),
                                force_add_effects_method.get_sig(),
                                &[
                                    JValue::Object(
                                        self.env.new_object(self.mappings.get("MobEffectInstance").unwrap().get_class(), "(Laxc;I)V", &[{
                                            let mob_effects_class = self.mappings.get("MobEffects").unwrap();
                                            let glowing_effect_mappings = mob_effects_class.get_static_field("GLOWING").unwrap();
                                            self.env
                                                .get_static_field(
                                                    mob_effects_class.get_class(),
                                                    glowing_effect_mappings.get_name(),
                                                    glowing_effect_mappings.get_sig(),
                                                ).unwrap()
                                        }, JValue::from(40)]).unwrap()
                                    ),
                                    JValue::Object(player_living_entity.get_object().unwrap()),
                                ],
                            )
                            .unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}