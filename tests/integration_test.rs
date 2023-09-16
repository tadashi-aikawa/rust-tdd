use rust_tdd::dollar::Dollar;

#[test]
fn test_multiplication() {
    let mut five = Dollar::new(5);

    assert_eq!(five.times(2), Dollar::new(10));
    assert_eq!(five.times(3), Dollar::new(15));
}

#[test]
fn test_equality() {
    assert!(Dollar::new(5).equals(Dollar::new(5)));
    assert!(!Dollar::new(5).equals(Dollar::new(6)));
}
