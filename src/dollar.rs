pub struct Dollar {
    pub amount: i32,
}

impl Dollar {
    pub fn new(amount: i32) -> Self {
        Dollar { amount: 10 }
    }

    pub fn times(&self, multiplier: i32) {}
}
