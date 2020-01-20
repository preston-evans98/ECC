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
    // let r = FieldElement::new(8, 19);
    println!("{}", (-3 % 7));

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

#[test]
#[should_panic]
fn test_create_invalid_curvepoint() {
    let curve = EllipticCurve::new(0, 7, 223);
    CurvePoint::new(200, 119, &curve);
}

#[test]
fn test_create_valid_curvepoint() {
    let curve = EllipticCurve::new(0, 7, 223);
    CurvePoint::new(192, 105, &curve);
}

#[test]
fn test_curve_add() {
    let curve = EllipticCurve::new(0, 7, 223);
    let p1 = CurvePoint::new(192, 105, &curve);
    let p2 = CurvePoint::new(17, 56, &curve);
    assert_eq!(&p1 + &p2, CurvePoint::new(170, 142, &curve));
    assert_ne!(&p1 + &p2, CurvePoint::new(60, 139, &curve));
    assert_eq!(p1 + p2, CurvePoint::new(170, 142, &curve));
    let p3 = CurvePoint::new(192, 105, &curve);
    let p4 = CurvePoint::new(192, 105, &curve);
    assert_eq!(&p3 + &p3, CurvePoint::new(49, 71, &curve));
    assert_eq!(p3 + p4, CurvePoint::new(49, 71, &curve));
}

#[test]
fn test_curve_mul() {
    let curve = EllipticCurve::new(0, 7, 223);
    let p1 = CurvePoint::new(143, 98, &curve);
    let r1 = CurvePoint::new(64, 168, &curve);
    assert_eq!(&p1 * 2, r1);
    assert_eq!(p1 * 2, r1);
    let p2 = CurvePoint::new(192, 105, &curve);
    let r2 = CurvePoint::new(49, 71, &curve);
    assert_eq!(&p2 * 2, r2);
    assert_eq!(p2 * 2, r2);
    let p3 = CurvePoint::new(47, 71, &curve);
    let r3 = CurvePoint::new(36, 111, &curve);
    assert_eq!(&p3 * 2, r3);
    assert_eq!(p3 * 2, r3);
    let p4 = CurvePoint::new(47, 71, &curve);
    let r4 = CurvePoint::new(194, 51, &curve);
    assert_eq!(&p4 * 4, r4);
    assert_eq!(p4 * 4, r4);
    let p5 = CurvePoint::new(47, 71, &curve);
    let r5 = CurvePoint::new(116, 55, &curve);
    assert_eq!(&p5 * 8, r5);
    assert_eq!(p5 * 8, r5);
    let p6 = CurvePoint::new(47, 71, &curve);
    let r6 = CurvePoint::infinity(&curve);
    assert_eq!(&p6 * 21, r6);
    assert_ne!(&p6 * 21, r5);
    assert_eq!(p6 * 21, r6);
}
