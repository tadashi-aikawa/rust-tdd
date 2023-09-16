pub trait Money {
    fn get_type(&self) -> &str;
    fn get_amount(&self) -> i32;
    fn equals<T: Money>(&self, other: &T) -> bool {
        self.get_amount() == other.get_amount() && self.get_type() == other.get_type()
    }
}
