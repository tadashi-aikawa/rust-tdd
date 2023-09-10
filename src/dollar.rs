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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let mut five = Dollar::new(5);

        let product = five.times(2);
        assert_eq!(product.amount, 10);

        let product = five.times(3);
        assert_eq!(product.amount, 15);
    }
}
