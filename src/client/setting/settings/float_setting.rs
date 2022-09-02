use core::ops::RangeInclusive;
use crate::client::setting::BingusSetting;
use crate::client::setting::SettingValue;

#[derive(Debug, Clone)]
pub struct FloatSetting {
    value: f64,
    name: String,
    range: RangeInclusive<f64>,
}

impl FloatSetting {
    pub fn new_boxed(value: SettingValue, name: &str, range: RangeInclusive<f64>) -> Self {
        FloatSetting {
            value: value.try_into().unwrap(),
            name: name.to_string(),
            range,
        }
    }

    pub fn get_range(&self) -> RangeInclusive<f64> {
        self.range.clone()
    }

    pub fn get_value_mut(&mut self) -> &mut f64 {
        &mut self.value
    }
}

impl BingusSetting for FloatSetting {
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

impl TryInto<String> for FloatSetting {
    type Error = ();
    fn try_into(self) -> Result<String, Self::Error> {
        Ok(self.value.to_string())
    }
}