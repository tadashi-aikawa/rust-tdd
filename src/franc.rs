use std::ops::Deref;

use crate::money::Money;

#[derive(Eq, PartialEq, Debug)]
pub struct Franc(Money);

impl Franc {
    pub fn new(amount: i32) -> Self {
        Franc(Money { amount })
    }

    pub fn times(&mut self, multiplier: i32) -> Self {
        Franc::new(self.amount * multiplier)
    }
}

impl Deref for Franc {
    type Target = Money;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
