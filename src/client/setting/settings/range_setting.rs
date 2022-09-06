use std::ops::RangeInclusive;

use crate::client::setting::{BingusSetting, SettingValue};

#[derive(Debug, Clone)]
pub struct RangeSetting {
    value: [f64; 2],
    range: RangeInclusive<f64>,
    name: String,
}

impl RangeSetting {
    pub fn new(value: SettingValue, range: RangeInclusive<f64>, name: &str) -> Self {
        RangeSetting {
            value: value.try_into().unwrap(),
            range,
            name: name.to_string(),
        }
    }

    pub fn get_value_mut(&mut self) -> &mut [f64; 2] {
        &mut self.value
    }

    pub fn get_range(&self) -> RangeInclusive<f64> {
        self.range.clone()
    }
}

impl BingusSetting for RangeSetting {
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

impl TryInto<String> for RangeSetting {
    type Error = ();
    fn try_into(self) -> Result<String, Self::Error> {
        Ok({
            format!("{}..={}", self.value[0], self.value[1])
        })
    }
}
