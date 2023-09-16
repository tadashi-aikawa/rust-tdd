use crate::money::Money;

#[derive(Eq, PartialEq, Debug)]
pub struct Dollar {
    amount: i32,
}

impl Money for Dollar {
    fn new(amount: i32) -> Self {
        Dollar { amount }
    }
    fn get_type(&self) -> &str {
        "dollar"
    }
    fn get_amount(&self) -> i32 {
        self.amount
    }
}
