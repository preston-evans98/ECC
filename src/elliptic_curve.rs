use crate::finite_field::FieldElement;
use num_bigint::{BigUint, ToBigUint};

#[derive(Debug, Clone)]
pub struct EllipticCurve {
    a: FieldElement,
    b: FieldElement,
}

#[derive(Debug)]
pub struct CurvePoint<'a> {
    is_infinity: bool,
    x: FieldElement,
    y: FieldElement,
    curve: &'a EllipticCurve,
}

impl EllipticCurve {
    pub fn new<T, U, V>(a: T, b: U, prime: V) -> EllipticCurve
    where
        T: ToBigUint,
        U: ToBigUint,
        V: ToBigUint + Clone,
    {
        EllipticCurve {
            a: FieldElement::new(a, prime.clone()),
            b: FieldElement::new(b, prime),
        }
    }
    pub fn new_large(a: &[u8], b: &[u8], prime: &[u8]) -> EllipticCurve {
        let a = BigUint::parse_bytes(a, 16).unwrap();
        let b = BigUint::parse_bytes(b, 16).unwrap();
        let prime = BigUint::parse_bytes(prime, 16).unwrap();
        EllipticCurve::new(a, b, prime)
    }

    pub fn get_prime(&self) -> &BigUint {
        self.a.get_prime()
    }
    pub fn get_a(&self) -> &BigUint {
        self.a.get_value()
    }
    pub fn get_b(&self) -> &BigUint {
        self.b.get_value()
    }
    pub fn is_valid(&self, x: &FieldElement, y: &FieldElement) -> bool {
        y.pow(2) == x.pow(3) + &self.a * x + &self.b
    }
}

impl<'a> CurvePoint<'a> {
    pub fn new<T, U>(x: T, y: U, curve: &'a EllipticCurve) -> CurvePoint
    where
        T: ToBigUint + std::fmt::Debug,
        U: ToBigUint + std::fmt::Debug,
    {
        let new = CurvePoint {
            is_infinity: false,
            x: FieldElement::new(x, curve.get_prime().clone()),
            y: FieldElement::new(y, curve.get_prime().clone()),
            curve,
        };
        if !new.is_infinity() && !curve.is_valid(&new.x, &new.y) {
            panic!(
                "Point ({:?}, {:?}) is not on curve y**2 = x**3 + {}*x + {}",
                new.x.get_value(),
                new.y.get_value(),
                curve.get_a(),
                curve.get_b()
            );
        }
        new
    }
    pub fn new_large(x: &[u8], y: &[u8], curve: &'a EllipticCurve) -> CurvePoint<'a> {
        let x = BigUint::parse_bytes(x, 16).unwrap();
        let y = BigUint::parse_bytes(y, 16).unwrap();
        CurvePoint::new(x, y, curve)
    }
    pub fn from(other: &'a CurvePoint) -> CurvePoint<'a> {
        if other.is_infinity() {
            return CurvePoint::infinity(other.curve);
        }
        CurvePoint::new(other.x.clone(), other.y.clone(), other.curve)
    }
    pub fn is_infinity(&self) -> bool {
        self.is_infinity
    }
    pub fn get_x(&self) -> &FieldElement {
        &self.x
    }
    pub fn get_y(&self) -> &FieldElement {
        &self.y
    }
    pub fn get_curve(&self) -> &'a EllipticCurve {
        self.curve
    }
    pub fn infinity(curve: &'a EllipticCurve) -> CurvePoint {
        CurvePoint {
            is_infinity: true,
            x: FieldElement::new(0, curve.get_prime().clone()),
            y: FieldElement::new(0, curve.get_prime().clone()),
            curve,
        }
    }
    #[allow(non_snake_case)]
    pub fn verify_signature(&self, z: &[u8], r: &[u8], s: &[u8], G: &CurvePoint, N: &BigUint) -> bool {
        let z = BigUint::parse_bytes(z, 16).unwrap();
        let r = BigUint::parse_bytes(r, 16).unwrap();
        let s = BigUint::parse_bytes(s, 16).unwrap();

        let s_inv = s.modpow(&(N - 2.to_biguint().unwrap()), N);
        let u = &z * &s_inv % N;
        let v = &r * &s_inv % N;

        return (G * u + self * v).get_x().get_value() == &r
    }
}

impl PartialEq for EllipticCurve {
    fn eq(&self, other: &Self) -> bool {
        self.get_a() == other.get_a() && self.get_b() == other.get_b()
    }
}

impl<'a> PartialEq for CurvePoint<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.get_curve() != other.get_curve() {
            return false;
        }
        if self.is_infinity() {
            return other.is_infinity();
        }
        if other.is_infinity() {
            return false;
        }
        self.get_x() == other.get_x() && self.get_y() == other.get_y()
    }
}

impl<'a> std::ops::Add for CurvePoint<'a> {
    type Output = CurvePoint<'a>;
    fn add(self, other: Self) -> CurvePoint<'a> {
        if self.get_curve() != other.get_curve() {
            panic!(
                "Addition Error: Points {:?} and {:?} are not on the same curve",
                self, other
            );
        }
        if other.is_infinity() {
            return self;
        } else if self.is_infinity() {
            return other;
        }
        let x1 = self.get_x();
        let x2 = other.get_x();
        let y1 = self.get_y();
        let y2 = other.get_y();
        let slope = {
            // If the points share an x-coordinate, we have two special cases
            if x1 == x2 {
                // Case 1: If they lie on a vertical line they add to infinity
                if y1 != y2 {
                    return CurvePoint::infinity(self.get_curve());
                }
                // Case 2: they're the same point. Take the derivative to find tangent line at that point
                // y**2 = x**3 + ax + b => 2y * dy = 3(x**2) + a => dy / dx = 3(x**2) + a / 2y
                // Thus slope = (3(x**2) + a) / 2y. Note that if 7=0, slope is infinite, so we should return the point at infinity
                if y1.get_value() == &0.to_biguint().unwrap() {
                    return CurvePoint::infinity(self.get_curve());
                }
                let mut numerator = x1.pow(2) * 3; // 3 x**2
                numerator.add_int_ref(self.get_curve().get_a()); // 3x**2 + a
                numerator / (y1 * 2)
            } else {
                // Otherwise slope is rise over run
                (y2 - y1) / (x2 - x1)
            }
        };
        // x3 = slope**2 - x1 - x2. Proof too large for comments here. See Readme
        let x3 = slope.pow(2) - x1 - x2;
        // y3 = slope * (x1 - x3) - y1
        let y3 = (slope * (x1 - &x3)) - y1;
        CurvePoint::new(x3, y3, self.get_curve())
    }
}

impl<'a> std::ops::Add for &CurvePoint<'a> {
    type Output = CurvePoint<'a>;
    fn add(self, other: Self) -> CurvePoint<'a> {
        if self.get_curve() != other.get_curve() {
            panic!(
                "Addition Error: Points {:?} and {:?} are not on the same curve",
                self, other
            );
        }
        if other.is_infinity() {
            return CurvePoint::new(self.get_x(), self.get_y(), self.get_curve());
        } else if self.is_infinity() {
            return CurvePoint::new(other.get_x(), other.get_y(), self.get_curve());
        }

        let x1 = self.get_x();
        let x2 = other.get_x();
        let y1 = self.get_y();
        let y2 = other.get_y();
        let slope = {
            // If the points share an x-coordinate, we have two special cases
            if x1 == x2 {
                // Case 1: If they lie on a vertical line they add to infinity
                if y1 != y2 {
                    return CurvePoint::infinity(self.get_curve());
                }
                // Case 2: they're the same point. Take the derivative to find tangent line at that point
                // y**2 = x**3 + ax + b => 2y * dy = 3(x**2) + a => dy / dx = 3(x**2) + a / 2y
                // Thus slope = (3(x**2) + a) / 2y
                let mut numerator = x1.pow(2) * 3; // 3 x**2
                numerator.add_int_ref(self.get_curve().get_a()); // 3x**2 + a
                numerator / (y1 * 2)
            } else {
                // Otherwise slope is rise over run
                (y2 - y1) / (x2 - x1)
            }
        };
        // x3 = slope**2 - x1 - x2. Proof too large for comments here. See Readme
        let x3 = slope.pow(2) - x1 - x2;
        // y3 = slope * (x1 - x3) - y1
        let y3 = (slope * (x1 - &x3)) - y1;
        CurvePoint::new(x3, y3, self.get_curve())
    }
}

impl<'a, Rhs> std::ops::Mul<Rhs> for CurvePoint<'a>
where
    Rhs: ToBigUint,
{
    type Output = CurvePoint<'a>;
    fn mul(self, rhs: Rhs) -> CurvePoint<'a> {
        let zero = 0.to_biguint().unwrap();
        let one = 1.to_biguint().unwrap();
        let mut rhs = rhs.to_biguint().unwrap();
        let mut current = CurvePoint::new(self.get_x(), self.get_y(), self.get_curve());
        let mut result = CurvePoint::infinity(&self.curve);
        while rhs != zero {
            if &rhs & &one != zero {
                result = &result + &current;
            }
            current = &current + &current;
            rhs >>= 1;
        }
        result
    }
}

impl<'a, Rhs> std::ops::Mul<Rhs> for &CurvePoint<'a>
where
    Rhs: ToBigUint,
{
    type Output = CurvePoint<'a>;
    fn mul(self, rhs: Rhs) -> CurvePoint<'a> {
        let zero = 0.to_biguint().unwrap();
        let one = 1.to_biguint().unwrap();
        let mut rhs = rhs.to_biguint().unwrap();
        let mut current = CurvePoint::new(self.get_x(), self.get_y(), self.get_curve());
        let mut result = CurvePoint::infinity(&self.curve);
        while rhs != zero {
            if &rhs & &one != zero {
                result = &result + &current;
            }
            current = &current + &current;
            rhs >>= 1;
        }
        result
    }
}
