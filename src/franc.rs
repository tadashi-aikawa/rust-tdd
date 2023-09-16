#[derive(Debug)]
pub struct Franc {
    amount: i32,
}

pub trait Money {
    fn get_amount(&self) -> i32;
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

impl<T: Money> PartialEq<T> for Franc {
    fn eq(&self, other: &T) -> bool {
        self.amount == other.get_amount()
    }
}

impl Money for Franc {
    fn get_amount(&self) -> i32 {
        self.amount
    }
}
