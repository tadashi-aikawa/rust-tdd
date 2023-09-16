use crate::dollar::Dollar;

pub trait Money {
    fn new(amount: i32) -> Self;

    fn get_type(&self) -> &str;
    fn get_amount(&self) -> i32;

    fn equals<T: Money>(&self, other: &T) -> bool {
        self.get_amount() == other.get_amount() && self.get_type() == other.get_type()
    }

    fn times(&self, multiplier: i32) -> Self
    where
        Self: Sized,
    {
        Self::new(self.get_amount() * multiplier)
    }
}
