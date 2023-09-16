use crate::money::Money;

#[derive(Eq, PartialEq, Debug)]
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

impl Money for Dollar {
    fn get_type(&self) -> &str {
        "dollar"
    }
    fn get_amount(&self) -> i32 {
        self.amount
    }
}
