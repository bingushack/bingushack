use crate::client::mapping::CM;
use jni::JNIEnv;
use std::{collections::HashMap, rc::Rc};

// todo make all the static stuff truly static with a Once and shit so i dont get them all repeatedly

#[derive(Debug, Default)]
pub struct MappingsManager<'a> {
    mappings: HashMap<String, CM<'a>>,
}

impl<'j> MappingsManager<'j> {
    pub fn new(jni_env: Rc<JNIEnv<'j>>) -> MappingsManager<'j> {
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

                adds!(cm);
                $fields_and_methods

                $new_self.mappings.insert($class_name.to_string(), cm);
            }}
        }

        let mut new_self = MappingsManager::default();

        // add the mappings
        add_mapping!(new_self, "MinecraftClient", "dyr", {
            add_field!("player", "s", "Lepw;", false);
            add_field!("level", "r", "Lems;", false);
            add_field!("interactionManager", "q", "Lemv;", false);

            add_method!("getInstance", "D", "()Ldyr;", true);
        });
        add_mapping!(new_self, "ClientLevel", "ems", {
            add_field!("players", "y", "()Ljava/util/List;", false);
        });
        add_mapping!(new_self, "PlayerEntity", "boj", {
            add_field!("currentScreenHandler", "bV", "Lbqp;", false);

            add_method!("getInventory", "fr", "()Lboi;", false);
            add_method!("getOffHandStack", "et", "()Lbuw;", false);
        });
        add_mapping!(new_self, "Inventory", "awa", {
            add_method!("getStack", "a", "(I)Lbuw;", false);
        });
        add_mapping!(new_self, "InteractionManager", "emv", {
            add_method!("clickSlot", "a", "(IIILbqy;Lboj;)V", false);
        });
        add_mapping!(new_self, "ScreenHandler", "bqp", {
            add_field!("syncId", "j", "I", false);
        });
        add_mapping!(new_self, "SlotActionType", "bqy", {
            add_field!("PICKUP", "a", "Lbqy;", true);
        });
        add_mapping!(new_self, "Items", "buy", {
            add_field!("TOTEM_OF_UNDYING", "sw", "Lbus;", true);
        });
        add_mapping!(new_self, "ItemStack", "buw", {
            add_method!("getItem", "c", "()Lbus;", false);
        });
        add_mapping!(new_self, "Item", "bus", {
            add_method!("getRawId", "a", "(Lbus;)I", true);
        });

        new_self
    }

    pub fn get(&self, class_name: &str) -> Option<&CM> {
        unsafe {
            self.mappings
                .get(&class_name.to_string())
                .map(|r| std::mem::transmute::<&CM<'j>, &CM<'_>>(r))
        }
    }
}
