use crate::finite_field::FieldElement;
use num_bigint::{BigUint, ToBigUint};

pub struct EllipticCurve {
    a: FieldElement,
    b: FieldElement,
}

pub struct CurvePoint<'a> {
    x: FieldElement,
    y: FieldElement,
    curve: &'a EllipticCurve,
}

impl EllipticCurve {
    pub fn new<T, U, V>(a: T, b: U, prime: V) -> EllipticCurve 
        where T: ToBigUint, U: ToBigUint, V: ToBigUint + Clone{
        EllipticCurve {
            a: FieldElement::new(a, prime.clone()),
            b: FieldElement::new(b, prime),
        }
    }
    pub fn get_prime(&self) -> BigUint {
        self.a.get_prime().clone()
    }
    // pub fn is_valid(x: F, y: BigUint) -> bool {
    //     y.pow(2) == pow(x, 3) + self.a * x + self.b
    // }
}

impl<'a> CurvePoint<'a > {
    pub fn new<T, U>(x: T, y: U, curve: &'a EllipticCurve) -> CurvePoint 
        where T: ToBigUint, U: ToBigUint {
        CurvePoint {
            x: FieldElement::new(x, curve.get_prime()),
            y: FieldElement::new(y, curve.get_prime()),
            curve
        }
    }
}
