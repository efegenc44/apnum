use apnum::{APNum, BigNat};

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
}
