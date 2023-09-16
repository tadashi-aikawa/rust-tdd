use rust_tdd::{dollar::Dollar, franc::Franc, money::Money};

#[test]
fn test_multiplication() {
    let five = Dollar::new(5);

    assert_eq!(five.times(2), Dollar::new(10));
    assert_eq!(five.times(3), Dollar::new(15));
}

#[test]
fn test_equality() {
    assert!(Dollar::new(5).equals(&Dollar::new(5)));
    assert!(!Dollar::new(5).equals(&Dollar::new(6)));
    assert!(Franc::new(5).equals(&Franc::new(5)));
    assert!(!Franc::new(5).equals(&Franc::new(6)));
    assert!(!Franc::new(5).equals(&Dollar::new(5)));
}

#[test]
fn test_franc_multiplication() {
    let five = Franc::new(5);

    assert_eq!(five.times(2), Franc::new(10));
    assert_eq!(five.times(3), Franc::new(15));
}
