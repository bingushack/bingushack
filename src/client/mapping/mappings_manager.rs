use crate::client::mapping::{CM, Mem, StaticMem};
use std::collections::HashMap;
use std::cell::RefCell;
use jni::{JNIEnv, JavaVM};

#[derive(Debug)]
pub struct MappingsManager<'a> {
    mappings: HashMap<String, CM<'a>>,
}

impl<'j> MappingsManager<'j> {
    pub fn new(jni_env: JNIEnv<'j>) -> MappingsManager<'j> {
        let mut new_self = MappingsManager {
            mappings: HashMap::new(),
        };

        new_self.map_stuff(jni_env);

        new_self
    }

    fn map_stuff(&mut self, jni_env: JNIEnv<'j>) {
        // todo make these into a macro for each mapping
        {
            let class_name =  "MinecraftClient".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("dyr").unwrap());

            Self::add_field(
                &mut cm,
                "player".to_string(),
                "s".to_string(),
                "Lepw;".to_string(),
                false,
            );

            Self::add_field(
                &mut cm,
                "level".to_string(),
                "r".to_string(),
                "Lems;".to_string(),
                false,
            );

            Self::add_method(
                &mut cm,
                "getInstance".to_string(),
                "D".to_string(),
                "()Ldyr;".to_string(),
                true,
            );

            Self::add_field(
                &mut cm,
                "interactionManager".to_string(),
                "q".to_string(),
                "Lemv;".to_string(),
                false,
            );


            self.mappings.insert(class_name, cm);
        }
        {
            let class_name = "ClientLevel".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("ems").unwrap());

            Self::add_method(
                &mut cm,
                "players".to_string(),
                "y".to_string(),
                "()Ljava/util/List;".to_string(),
                false,
            );

            self.mappings.insert(class_name, cm);
        }
        {
            let class_name = "PlayerEntity".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("boj").unwrap());

            Self::add_method(
                &mut cm,
                "getInventory".to_string(),
                "fr".to_string(),
                "()Lboi;".to_string(),
                false,
            );

            Self::add_field(
                &mut cm,
                "currentScreenHandler".to_string(),
                "bV".to_string(),
                "Lbqp;".to_string(),
                false,
            );

            Self::add_method(
                &mut cm,
                "getOffHandStack".to_string(),
                "et".to_string(),
                "()Lbuw;".to_string(),
                false,
            );

            self.mappings.insert(class_name, cm);
        }
        {
            let class_name = "Inventory".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("awa").unwrap());  // net/minecraft/world/Container
            // finish
            Self::add_method(
                &mut cm,
                "getStack".to_string(),
                "a".to_string(),
                "(I)Lbuw;".to_string(),
                false,
            );
            self.mappings.insert(class_name, cm);
        }
        {
            let class_name = "InteractionManager".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("emv").unwrap());
            Self::add_method(
                &mut cm,
                "clickSlot".to_string(),
                "a".to_string(),
                "(IIILbqy;Lboj;)V".to_string(),  // int syncId, int slotId, int button, SlotActionType actionType, PlayerEntity player
                false,
            );

            self.mappings.insert(class_name, cm);
        }
        {
            let class_name = "ScreenHandler".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("bqp").unwrap());

            Self::add_field(
                &mut cm,
                "syncId".to_string(),
                "j".to_string(),
                "I".to_string(),
                false,
            );

            self.mappings.insert(class_name, cm);
        }
        {
            let class_name = "SlotActionType".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("bqy").unwrap());
            Self::add_field(
                &mut cm,
                "PICKUP".to_string(),
                "a".to_string(),
                "Lbqy;".to_string(),
                true,
            );

            self.mappings.insert(class_name, cm);
        }
        {
            let class_name = "Items".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("buy").unwrap());
            Self::add_field(
                &mut cm,
                "TOTEM_OF_UNDYING".to_string(),
                "sw".to_string(),
                "Lbus;".to_string(),
                true,
            );
            self.mappings.insert(class_name, cm);
        }
        {
            let class_name = "ItemStack".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("buw").unwrap());
            Self::add_method(
                &mut cm,
                "getItem".to_string(),
                "c".to_string(),
                "()Lbus;".to_string(),
                false,
            );
            self.mappings.insert(class_name, cm);
        }
        {
            let class_name = "Item".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("bus").unwrap());

            Self::add_method(
                &mut cm,
                "getRawId".to_string(),
                "a".to_string(),
                "(Lbus;)I".to_string(),
                true,
            );

            self.mappings.insert(class_name, cm);
        }
    }

    pub fn get(&self, class_name: &str) -> Option<&CM> {
        unsafe { self.mappings.get(&class_name.to_string()).map(|r| std::mem::transmute::<&CM<'j>, &CM<'_>>(r)) }
    }

    // todo move to CM impl block
    fn make<'m>() -> CM<'m> {
        CM {
            class: RefCell::new(None),
            object: RefCell::new(None),

            fields: HashMap::new(),
            static_fields: HashMap::new(),

            methods: HashMap::new(),
            static_methods: HashMap::new()
        }
    }

    fn add_field(
        cm: &mut CM,
        key_name: String,
        ob_name: String,
        sig: String,
        is_static: bool,
    ) {
        let m = Mem {
            name: ob_name,
            sig,
        };

        if is_static {
            cm.static_fields.insert(key_name, StaticMem { mem: m } );
        } else {
            cm.fields.insert(key_name, m);
        }
    }

    fn add_method(
        cm: &mut CM,
        key_name: String,
        ob_name: String,
        sig: String,
        is_static: bool,
    ) {
        let m = Mem {
            name: ob_name,
            sig,
        };

        if is_static {
            cm.static_methods.insert(key_name, StaticMem { mem: m } );
        } else {
            cm.methods.insert(key_name, m);
        }
    }
}
