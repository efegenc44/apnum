use apnum::{APNum, BigInt};

// These tests are essentailly for testing sign calculations.
// All numeric calculations based on BigNat which is tested

#[test]
fn bigint_add() {
    let x = BigInt::zero();
    let y = BigInt::from(123);
    assert_eq!(&x + &y, y);
    let x = BigInt::from(321);
    let y = BigInt::from(-296);
    assert_eq!(x + y, BigInt::from(25));
    let x = BigInt::from(-321);
    let y = BigInt::from(12);
    assert_eq!(x + y, BigInt::from(-309));
    let x = BigInt::from(77);
    let y = BigInt::from(-33);
    assert_eq!(x + y, BigInt::from(44));
}

#[test]
fn bigint_mul() {
    let x = BigInt::zero();
    let y = BigInt::from(123);
    assert_eq!(&x * &y, BigInt::zero());
    let x = BigInt::from(321);
    let y = BigInt::from(-296);
    assert_eq!(x * y, BigInt::from(-95016));
    let x = BigInt::from(-321);
    let y = BigInt::from(12);
    assert_eq!(x * y, BigInt::from(-3852));
    let x = BigInt::from(-77);
    let y = BigInt::from(-33);
    assert_eq!(x * y, BigInt::from(2541));
}

#[test]
fn bigint_sub() {
    let x = BigInt::from(100);
    let y = BigInt::from(-98);
    assert_eq!(&x - &y, BigInt::from(198));
    assert_eq!(&y - &x, BigInt::from(-198));
    assert_eq!(&x - &x, BigInt::zero());

    let x = BigInt::from(4464);
    let y = BigInt::from(-18);
    assert_eq!(&x - &y, BigInt::from(4482));
    assert_eq!(&y - &x, BigInt::from(-4482));

    let x = BigInt::from(5);
    assert_eq!(&x - &BigInt::zero(), BigInt::from(5));
    assert_eq!(&BigInt::zero() - &x, BigInt::from(-5));

    assert_eq!(BigInt::zero() - BigInt::zero(), BigInt::zero());
}

#[test]
fn bigint_div() {
    let x = BigInt::from(-1000);
    let y = BigInt::from(900);
    assert_eq!(&x / &y, (BigInt::from(-2), BigInt::from(800)));
    let x = BigInt::from(42);
    let y = BigInt::from(-10);
    assert_eq!(&x / &y, (BigInt::from(-5), BigInt::from(-8)));
    let x = BigInt::from(43);
    let y = BigInt::from(-2);
    assert_eq!(&x / &y, (BigInt::from(-22), BigInt::from(-1)));
    let x = BigInt::from(-789);
    let y = BigInt::from(-34);
    assert_eq!(&x / &y, (BigInt::from(23), BigInt::from(-7)));
    assert_eq!(&-x / &y, (BigInt::from(-24), BigInt::from(-27)));
    let x = BigInt::from(0);
    let y = BigInt::from(2);
    assert_eq!(&x / &y, (BigInt::zero(), BigInt::zero()));
}

#[test]
fn bigint_cmp() {
    let x = BigInt::from(123);
    let y = BigInt::from(321);
    assert!(x < y);
    assert!(-x < y);
    let x = BigInt::from(123);
    let y = BigInt::from(21);
    assert!(x > y);
    assert!(-x < -y);
    let x = BigInt::from(4);
    let y = BigInt::from(4);
    assert!(x == y);
    assert!(x > -y);
    let x = BigInt::from(0);
    let y = BigInt::from(0);
    assert!(x == y);
    assert!(x == -y);
    let x = BigInt::from(0);
    let y = BigInt::from(4);
    assert!(x < y);
    assert!(x > -y);
}
