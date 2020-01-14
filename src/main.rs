// use num_bigint::{BigUint};
extern crate num_integer;
// use num_integer::Integer;

mod elliptic_curve;
mod finite_field;
use elliptic_curve::{CurvePoint, EllipticCurve};
use finite_field::FieldElement;
// use finiteField;

fn main() {
    // let n = BigUint::parse_bytes(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap();
    let curve = EllipticCurve::new(0, 7, 223);
    // let secp256k = EllipticCurve::new(0, 7, 0xfffffff);
    let _generator = CurvePoint::new(192, 105, &curve);
    let r = FieldElement::new(8, 19);
    println!("{}", (r / 2));

    // println!("{}", n - 518161494337 as u64);
}

#[cfg(test)]
#[test]
fn test_ne_fieldelement() {
    let a = FieldElement::new(2, 31);
    let b = FieldElement::new(2, 31);
    let c = FieldElement::new(15, 31);
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert!(!(a != b))
}

#[test]
fn test_add_fieldelement() {
    let mut a = FieldElement::new(2, 31);
    let b = FieldElement::new(15, 31);
    a += b;
    assert_eq!(a, FieldElement::new(17, 31));
    let c = FieldElement::new(21, 31);
    let d = &a + &c;
    assert_eq!(d, FieldElement::new(7, 31));
}

#[test]
fn test_sub_fieldelement() {
    let mut a = FieldElement::new(29, 31);
    let b = FieldElement::new(4, 31);
    a -= &b;
    assert_eq!(a, FieldElement::new(25, 31));
    let c = FieldElement::new(15, 31);
    let d = FieldElement::new(30, 31);
    assert_eq!(&c - &d, FieldElement::new(16, 31))
}

#[test]
fn test_mul_fieldelement() {
    let mut a = FieldElement::new(24, 31);
    let b = FieldElement::new(19, 31);
    assert_eq!(&a * &b, FieldElement::new(22, 31));
    a *= b;
    assert_eq!(a, FieldElement::new(22, 31));
}
#[test]
fn test_pow_fieldelement() {
    let a = FieldElement::new(17, 31);
    assert_eq!(a.pow(3), FieldElement::new(15, 31));
    let b = FieldElement::new(18, 31);
    let c = FieldElement::new(5, 31);
    assert_eq!(c.pow(5) * b, FieldElement::new(16, 31));
}

#[test]
fn test_div_fieldelement() {
    let mut a = FieldElement::new(3, 31);
    let b = FieldElement::new(24, 31);
    a /= b;
    assert_eq!(a, FieldElement::new(4, 31));
    let c = FieldElement::new(17, 31);
    assert_eq!(c.pow(-3), FieldElement::new(29, 31));
}
