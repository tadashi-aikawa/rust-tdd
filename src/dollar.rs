#[derive(PartialEq, Debug)]
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
