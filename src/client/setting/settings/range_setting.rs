use std::ops::RangeInclusive;
use rand::Rng;

use crate::client::setting::{BingusSetting, SettingValue};

#[derive(Debug, Clone)]
pub struct RangeSetting {
    value: [f64; 2],
    range: RangeInclusive<f64>,
    max_decimals: Option<usize>,
    step_by: Option<f64>,
    name: String,
}

impl RangeSetting {
    pub fn new(
        value: SettingValue,
        range: RangeInclusive<f64>,
        max_decimals: Option<usize>,
        step_by: Option<f64>,
        name: &str
    ) -> Self {
        RangeSetting {
            value: value.try_into().unwrap(),
            range,
            max_decimals,
            step_by,
            name: name.to_string(),
        }
    }

    pub fn get_value_mut(&mut self) -> &mut [f64; 2] {
        &mut self.value
    }

    pub fn get_random_f64_in_range(&self) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.value[0]..=self.value[1])
    }

    pub fn get_random_usize_in_range(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.value[0] as usize..=self.value[1] as usize)
    }

    pub fn get_range(&self) -> RangeInclusive<f64> {
        self.range.clone()
    }

    pub fn get_max_decimals(&self) -> Option<usize> {
        self.max_decimals
    }

    pub fn get_step_by(&self) -> Option<f64> {
        self.step_by
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
