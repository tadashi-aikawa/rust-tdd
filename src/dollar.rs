use std::ops::Deref;

use crate::money::Money;

#[derive(Eq, PartialEq, Debug)]
pub struct Dollar(Money);

impl Dollar {
    pub fn new(amount: i32) -> Self {
        Dollar(Money { amount })
    }

    pub fn times(&mut self, multiplier: i32) -> Self {
        Dollar::new(self.amount * multiplier)
    }
}

impl Deref for Dollar {
    type Target = Money;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
