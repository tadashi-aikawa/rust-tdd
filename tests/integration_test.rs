use rust_tdd::dollar::Dollar;

#[test]
fn test_multiplication() {
    let mut five = Dollar::new(5);

    let product = five.times(2);
    assert_eq!(product.amount, 10);

    let product = five.times(3);
    assert_eq!(product.amount, 15);
}