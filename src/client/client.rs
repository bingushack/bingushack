use crate::{client::module::modules::*, managers::ModulesRc};

use super::{mapping::*, module::*};
use jni::JNIEnv;
use std::{
    rc::Rc,
    cell::RefCell, sync::Mutex,
};

pub type BoxedBingusModule = Box<&'static dyn BingusModule>;
