pub struct Dollar {
    pub amount: i32,
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

    pub fn equals(&mut self, other: Self) -> bool {
        self.amount == other.amount
    }
}
