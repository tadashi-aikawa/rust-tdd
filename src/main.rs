mod dollar;

use crate::dollar::Dollar;
use std::ops::Range;

fn main() {
    let xs: Range<i32> = -10..10;
    let result: i32 = xs.filter(|x| x.is_positive()).map(|x| x * x).sum();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        five.times(2);
        assert_eq!(five.amount, 10);
    }
}
