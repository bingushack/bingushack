use crate::client::setting::{BingusSetting, SettingValue};

#[derive(Debug, Clone)]
pub struct BooleanSetting {
    value: bool,
    name: String,
}

impl BooleanSetting {
    pub fn new(value: SettingValue, name: &str) -> Self {
        BooleanSetting {
            value: value.try_into().unwrap(),
            name: name.to_string(),
        }
    }

    pub fn get_value_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}

impl BingusSetting for BooleanSetting {
    fn get_value(&self) -> SettingValue {
        SettingValue::from(self.value)
    }

    fn set_value(&mut self, new_value: SettingValue) {
        self.value = new_value.try_into().unwrap();
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

impl TryInto<String> for BooleanSetting {
    type Error = ();
    fn try_into(self) -> Result<String, Self::Error> {
        Ok(self.value.to_string())
    }
}
