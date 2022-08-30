use crate::client::setting::BingusSetting;
use crate::client::setting::SettingValue;
use crate::client::RcBoxedBingusSetting;
use std::rc::Rc;

pub struct BooleanSetting(bool);

impl BingusSetting for BooleanSetting {
    fn new_boxed(value: SettingValue) -> RcBoxedBingusSetting where Self: Sized {
        Rc::new(Box::new(BooleanSetting(value.try_into().unwrap())))
    }

    fn get_value(&self) -> SettingValue {
        SettingValue::from(self.0)
    }

    fn set_value(&mut self, new_value: SettingValue) {
        self.0 = new_value.try_into().unwrap();
    }
}

impl TryInto<String> for BooleanSetting {
    type Error = ();
    fn try_into(self) -> Result<String, Self::Error> {
        Ok(self.0.to_string())
    }
}
