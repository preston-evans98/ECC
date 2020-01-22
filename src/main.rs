use num_bigint::{BigUint, ToBigUint};
extern crate num_integer;
// use num_integer::Integer;
mod elliptic_curve;
mod finite_field;
use elliptic_curve::{CurvePoint, EllipticCurve};
use finite_field::FieldElement;
// use finiteField;

fn main() {
    let S256 = EllipticCurve::new_large(
        b"0",
        b"7",
        b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
    );
    let G = CurvePoint::new_large(
        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        &S256,
    );
    let N = BigUint::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap();
    // let z = BigUint::parse_bytes(b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423", 16).unwrap();
    // let r = BigUint::parse_bytes(b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6", 16).unwrap();
    // let s = BigUint::parse_bytes(b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec", 16).unwrap();
    let point = CurvePoint::new_large(
        b"887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c",
        b"61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34",
        &S256,
    );
    // // println!("{:?}", G * n);
    // let s_inv = s.modpow(&(&N - 2.to_biguint().unwrap()), &N); //pow(s, N-2, N)
    // let u = &z * &s_inv % &N;
    // let v = &r * &s_inv % &N;
    println!(
        "{}",
        point.verify_signature(
            b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
            b"eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
            b"c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
            &G,
            &N
        )
    );

    // assert_eq!(G * n, CurvePoint::infinity(&S256));
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

#[test]
fn test_secp256k1() {
    let S256 = EllipticCurve::new_large(
        b"0",
        b"7",
        b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
    );
    let G = CurvePoint::new_large(
        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        &S256,
    );
    let n = BigUint::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap();
    assert_eq!(G * n, CurvePoint::infinity(&S256));
}

#[test]
fn test_validate_signature() {
    let S256 = EllipticCurve::new_large(
        b"0",
        b"7",
        b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
    );
    let G = CurvePoint::new_large(
        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        &S256,
    );
    let N = BigUint::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap();
    let point = CurvePoint::new_large(
        b"887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c",
        b"61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34",
        &S256,
    );
    assert!(point.verify_signature(
        b"ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60",
        b"ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395",
        b"68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4",
        &G,
        &N
    ));
    assert!(point.verify_signature(
        b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
        b"eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
        b"c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
        &G,
        &N
    ));
}
