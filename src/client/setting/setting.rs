pub trait BingusSetting<T: ToString> {
    fn get_value(&self) -> T;

    fn set_value(&mut self, new_value: T);

    fn to_string(&self) -> String;
}