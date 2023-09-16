use crate::money::Money;

#[derive(Eq, PartialEq, Debug)]
pub struct Franc {
    amount: i32,
}

impl Money for Franc {
    fn new(amount: i32) -> Self {
        Franc { amount }
    }

    fn get_type(&self) -> &str {
        "franc"
    }
    fn get_amount(&self) -> i32 {
        self.amount
    }
}
