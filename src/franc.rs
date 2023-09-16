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