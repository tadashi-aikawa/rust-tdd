use crate::money::Money;

#[derive(Debug)]
pub struct Dollar {
    amount: i32,
}

impl Dollar {
    pub fn new(amount: i32) -> Self {
        Dollar { amount }
    }

    pub fn times(&mut self, multiplier: i32) -> Self {
        Dollar {
            amount: self.amount * multiplier,
        }
    }
}

impl<T: Money> PartialEq<T> for Dollar {
    fn eq(&self, other: &T) -> bool {
        self.amount == other.get_amount()
    }
}

impl Money for Dollar {
    fn get_amount(&self) -> i32 {
        self.amount
    }
}
