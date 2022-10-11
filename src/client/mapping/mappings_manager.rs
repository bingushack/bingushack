use crate::{client::mapping::CM, message_box};
use jni::JNIEnv;
use std::{collections::HashMap, rc::Rc};

// todo make all the static stuff truly static with a Once and shit so i dont get them all repeatedly

#[derive(Debug, Default)]
pub struct MappingsManager<'a> {
    mappings: HashMap<String, CM<'a>>,
}

impl<'j> MappingsManager<'j> {
    pub fn new(jni_env: Rc<JNIEnv<'j>>) -> MappingsManager<'j> {
        // macros to make stuff nice and easy(ish)
        macro_rules! adds {
            ($cm:ident) => {
                #[allow(unused_macros)]
                macro_rules! add_field {
                    ($key_name:literal, $ob_name:literal, $sig:literal, $is_static:literal) => {
                        $cm.add_field(
                            $key_name.to_string(),
                            $ob_name.to_string(),
                            $sig.to_string(),
                            $is_static,
                        )
                    };
                }

                #[allow(unused_macros)]
                macro_rules! add_method {
                    ($key_name:literal, $ob_name:literal, $sig:literal, $is_static:literal) => {
                        $cm.add_method(
                            $key_name.to_string(),
                            $ob_name.to_string(),
                            $sig.to_string(),
                            $is_static,
                        )
                    };
                }
            };
        }
        // macro for making a class mapping
        macro_rules! add_mapping {
            (
                $new_self:ident,
                $class_name:literal,            // the easy-to-use name of the class
                $class_path:literal,            // path to the class or the obfuscated class name
                $fields_and_methods:block       // the fields and methods of the class (using the `add_field_or_method!` macro)
            ) => {{
                let mut cm = CM::default();
                cm.apply_class(jni_env.find_class($class_path).unwrap());

                // didn't feel like making a proc_macro so this instead, def inflates binary size but whatever
                adds!(cm);
                $fields_and_methods

                $new_self.mappings.insert($class_name.to_string(), cm);
            }}
        }

        let mut new_self = MappingsManager::default();

        // add the mappings
        // https://wagyourtail.xyz/Projects/MinecraftMappingViewer/App
        add_mapping!(new_self, "MinecraftClient", "efu", {
            add_field!("player", "t", "Leyw;", false);
            add_field!("level", "s", "Leuv;", false);
            add_field!("interactionManager", "r", "Leuy;", false);

            add_method!("getInstance", "I", "()Lefu;", true);
            add_method!("getTickDelta", "am", "()F", false);
        });
        add_mapping!(new_self, "PlayerEntity", "boj", {
            add_field!("currentScreenHandler", "bU", "Lbwm;", false);

            add_method!("getInventory", "fA", "()Lbub;", false);
            add_method!("getOffHandStack", "eA", "()Lcax;", false);
            add_method!("isUsingItem", "eT", "()Z", false);
            add_method!("swingHand", "a", "(Lbai;Z)V", false);
            add_method!("getAttackCooldownProgress", "v", "(F)F", false);
        });
        add_mapping!(new_self, "Inventory", "bac", {
            add_method!("getStack", "a", "(I)Lcax;", false);
        });
        add_mapping!(new_self, "InteractionManager", "euy", {
            add_method!("clickSlot", "a", "(IIILbwv;Lbuc;)V", false);
            add_method!("attackEntity", "a", "(Lbuc;Lbbn;)V", false);
        });
        add_mapping!(new_self, "ScreenHandler", "bwm", {
            add_field!("syncId", "j", "I", false);
        });
        add_mapping!(new_self, "SlotActionType", "bwv", {
            add_field!("PICKUP", "a", "Lbwv;", true);
        });
        add_mapping!(new_self, "Items", "caz", {  // breaks
            add_field!("TOTEM_OF_UNDYING", "tn", "Lcat;", true);
        });
        add_mapping!(new_self, "ItemStack", "cax", {
            add_method!("getItem", "c", "()Lcat;", false);
        });
        add_mapping!(new_self, "Item", "cat", {
            add_method!("getRawId", "a", "(Lcat;)I", true);
        });
        add_mapping!(new_self, "Optional", "java/util/Optional", {
            add_method!("isPresent", "isPresent", "()Z", false);
            add_method!("get", "get", "()Ljava/lang/Object;", false);
        });
        add_mapping!(new_self, "DebugRenderer", "fcv", {
            add_method!("getTargetedEntity", "a", "(Lbbn;I)Ljava/util/Optional;", true);
        });
        add_mapping!(new_self, "Entity", "bbn", {
            add_method!("isAlive", "bo", "()Z", false);
        });
        add_mapping!(new_self, "Hand", "bai", {
            add_field!("MAIN_HAND", "a", "Lbai;", true);
        });

        new_self
    }

    pub fn get(&self, class_name: &str) -> Option<&CM> {
        unsafe {
            self.mappings
                .get(&class_name.to_string())
                .map(|r| std::mem::transmute::<&CM<'j>, &CM<'_>>(r))  // i don't know why this transmute is legal but it is so cope
        }
    }
}
