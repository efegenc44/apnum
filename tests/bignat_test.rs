use apnum::{APNum, BigInt, BigNat};

#[test]
fn bignat_add() {
    let x = BigNat::zero();
    let y = BigNat::from(123usize);
    assert_eq!(&x + &y, y);
    let x = BigNat::from(321usize);
    let y = BigNat::from(296usize);
    assert_eq!(x + y, BigNat::from(617usize));
    let x = BigNat::from(321usize);
    let y = BigNat::from(12usize);
    assert_eq!(x + y, BigNat::from(333usize));
    let x = BigNat::from(77usize);
    let y = BigNat::from(33usize);
    assert_eq!(x + y, BigNat::from(110usize));
}

#[test]
fn bignat_mul() {
    let x = BigNat::zero();
    let y = BigNat::from(123usize);
    assert_eq!(&x * &y, BigNat::zero());
    let x = BigNat::from(321usize);
    let y = BigNat::from(296usize);
    assert_eq!(x * y, BigNat::from(95016usize));
    let x = BigNat::from(321usize);
    let y = BigNat::from(12usize);
    assert_eq!(x * y, BigNat::from(3852usize));
    let x = BigNat::from(77usize);
    let y = BigNat::from(33usize);
    assert_eq!(x * y, BigNat::from(2541usize));
}

#[test]
fn bignat_sub() {
    let x = BigNat::from(100usize);
    let y = BigNat::from(98usize);
    assert_eq!(&x - &y, BigInt::from(2));
    assert_eq!(&y - &x, BigInt::from(-2));
    assert_eq!(&x - &x, BigInt::zero());

    let x = BigNat::from(4464usize);
    let y = BigNat::from(18usize);
    assert_eq!(&x - &y, BigInt::from(4446));
    assert_eq!(&y - &x, BigInt::from(-4446));

    let x = BigNat::from(5usize);
    assert_eq!(&x - &BigNat::zero(), BigInt::from(5));
    assert_eq!(&BigNat::zero() - &x, BigInt::from(-5));

    assert_eq!(BigNat::zero() - BigNat::zero(), BigInt::zero());
}

#[test]
fn bignat_div() {
    let x = BigNat::from(1000usize);
    let y = BigNat::from(900usize);
    assert_eq!(&x / &y, (BigNat::from(1usize), BigNat::from(100usize)));
    let x = BigNat::from(42usize);
    let y = BigNat::from(10usize);
    assert_eq!(&x / &y, (BigNat::from(4usize), BigNat::from(2usize)));
    let x = BigNat::from(43usize);
    let y = BigNat::from(2usize);
    assert_eq!(&x / &y, (BigNat::from(21usize), BigNat::from(1usize)));
    let x = BigNat::from(0usize);
    let y = BigNat::from(2usize);
    assert_eq!(&x / &y, (BigNat::zero(), BigNat::zero()));
    // see. Knuth, The Art Of Computer Programming Vol. 2 Section 4.3.1, Solution of Exercise 22
    let x = BigNat::from(4100usize);
    let y = BigNat::from(588usize);
    assert_eq!(&x / &y, (BigNat::from(6usize), BigNat::from(572usize)));
}

#[test]
fn bignat_cmp() {
    let x = BigNat::from(123usize);
    let y = BigNat::from(321usize);
    assert!(x < y);
    let x = BigNat::from(123usize);
    let y = BigNat::from(21usize);
    assert!(x > y);
    let x = BigNat::from(4usize);
    let y = BigNat::from(4usize);
    assert!(x == y);
    let x = BigNat::from(0usize);
    let y = BigNat::from(0usize);
    assert!(x == y);
    let x = BigNat::from(0usize);
    let y = BigNat::from(4usize);
    assert!(x < y);
}
