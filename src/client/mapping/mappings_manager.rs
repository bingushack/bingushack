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
            /*
            Self::add_field(
                &mut cm,
                "player".to_string(),
                "s".to_string(),
                "Lepw;".to_string(),
                false,
            );
            */

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

        // remove
        {
            let class_name = "List".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("java/util/List").unwrap());

            Self::add_method(
                &mut cm,
                "size".to_string(),
                "size".to_string(),
                "()I".to_string(),
                false,
            );

            Self::add_method(
                &mut cm,
                "get".to_string(),
                "get".to_string(),
                "(I)L".to_string(),
                false,
            );

            self.mappings.insert(class_name, cm);
        }
        {
            let class_name = "LivingEntity".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("axy").unwrap());

            Self::add_method(
                &mut cm,
                "forceAddEffect".to_string(),
                "c".to_string(),
                "(Laxe;Laxk;)V".to_string(),  // args are a `MobEffectInstance` and `Entity`
                false,
            );

            self.mappings.insert(class_name, cm);
        }
        // make an instance with `JNIEnv::new_object()` and a `JClass` of `MobEffects`
        {
            let class_name = "MobEffectInstance".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("axe").unwrap());

            self.mappings.insert(class_name, cm);
        }
        {
            let class_name = "MobEffects".to_string();
            let mut cm = Self::make();
            cm.apply_class(jni_env.find_class("axg").unwrap());

            Self::add_field(
                &mut cm,
                "GLOWING".to_string(),
                "x".to_string(),
                "Laxc;".to_string(),
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
            class: None,
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
