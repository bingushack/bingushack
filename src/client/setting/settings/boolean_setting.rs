use crate::client::setting::BingusSetting;

pub struct BooleanSetting(bool);

impl BingusSetting<bool> for BooleanSetting {
    fn get_value(&self) -> bool {
        self.0
    }

    fn set_value(&mut self, new_value: bool) {
        self.0 = new_value;
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
