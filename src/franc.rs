use crate::money::Money;

#[derive(Eq, PartialEq, Debug)]
pub struct Franc {
    amount: i32,
}

impl Franc {
    pub fn new(amount: i32) -> Self {
        Franc { amount }
    }

    pub fn times(&mut self, multiplier: i32) -> Self {
        Franc {
            amount: self.amount * multiplier,
        }
    }
}

impl Money for Franc {
    fn get_amount(&self) -> i32 {
        self.amount
    }
}
