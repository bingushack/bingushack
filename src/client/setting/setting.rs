// todo implement From trait for BingusSetting
pub trait BingusSetting {
    fn get_value(&self) -> SettingValue;

    fn set_value(&mut self, new_value: SettingValue);

    fn get_name(&self) -> &String;
}

pub enum SettingValue {
    Bool(bool),
    Int(i32),
    String(String),
    Float(f64),
}

impl TryInto<f64> for SettingValue {
    type Error = ();
    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            SettingValue::Float(f) => Ok(f),
            _ => Err(()),
        }
    }
}

impl From<f64> for SettingValue {
    fn from(f: f64) -> Self {
        SettingValue::Float(f)
    }
}

impl TryInto<bool> for SettingValue {
    type Error = ();
    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            SettingValue::Bool(b) => Ok(b),
            _ => Err(()),
        }
    }
}

impl From<bool> for SettingValue {
    fn from(b: bool) -> Self {
        SettingValue::Bool(b)
    }
}

impl TryInto<i32> for SettingValue {
    type Error = ();
    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            SettingValue::Int(i) => Ok(i),
            _ => Err(()),
        }
    }
}

impl From<i32> for SettingValue {
    fn from(i: i32) -> Self {
        SettingValue::Int(i)
    }
}

impl TryInto<String> for SettingValue {
    type Error = ();
    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            SettingValue::String(s) => Ok(s),
            SettingValue::Bool(b) => Ok(b.to_string()),
            SettingValue::Int(i) => Ok(i.to_string()),
            SettingValue::Float(f) => Ok(f.to_string()),
        }
    }
}

impl From<String> for SettingValue {
    fn from(s: String) -> Self {
        SettingValue::String(s)
    }
}
