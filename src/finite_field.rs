use num_bigint::{BigUint, ToBigInt, ToBigUint};
use num_integer::Integer;
use std::fmt;
// use std::ops::{Div, Sub, Rem};
// use std::cmp::PartialOrd;
// use num_integer::Integer;
#[derive(Debug)]
pub struct FieldElement {
    value: BigUint,
    prime: BigUint,
}
impl FieldElement {
    pub fn new<T, U>(value: T, prime: U) -> FieldElement
    where
        T: ToBigUint,
        U: ToBigUint,
    {
        let v = value.to_biguint().unwrap();
        let p = prime.to_biguint().unwrap();
        FieldElement {
            value: v % &p,
            prime: p,
        }
    }
    pub fn from(other: &FieldElement) -> FieldElement {
        FieldElement {
            value: other.get_value().clone(),
            prime: other.get_prime().clone(),
        }
    }
    pub fn add(&mut self, other: &FieldElement) {
        if self.prime != other.prime {
            panic!("Primes don't match!");
        }
        self.value += &other.value;
        self.value %= &self.prime;
    }
    pub fn get_value(&self) -> &BigUint {
        &self.value
    }
    pub fn get_prime(&self) -> &BigUint {
        &self.prime
    }
    pub fn square(&mut self) {
        self.value = (&self.value * &self.value) % &self.prime;
    }
    pub fn pow<T>(&self, exp: T) -> FieldElement
    where
        T: ToBigInt + ToBigUint,
    {
        let big_exponent = exp.to_bigint().unwrap();
        if big_exponent < 0.to_bigint().unwrap() {
            let power = -big_exponent;
            return FieldElement::new(1, self.prime.clone())
                / FieldElement::raise(&self, power.to_biguint().unwrap());
        }
        FieldElement::raise(&self, exp)
    }
    pub fn raise<T>(base: &FieldElement, exponent: T) -> FieldElement
    where
        T: ToBigUint,
    {
        let mut result = FieldElement::from(&base);
        let mut accumulator = FieldElement::new(1, base.get_prime().clone());
        let mut exp = exponent.to_biguint().unwrap();
        // let zero = 0.to_biguint().unwrap();
        let one = 1.to_biguint().unwrap();
        let two = 2.to_biguint().unwrap();
        while &exp > &one {
            if (&exp).is_even() {
                result.square();
                exp /= &two;
            } else {
                accumulator *= &result;
                exp -= &one;
            }
        }
        result *= accumulator;
        result
    }
}

impl ToBigUint for FieldElement {
    fn to_biguint(&self) -> Option<BigUint> {
        Some(self.value.clone())
    }
}

impl ToBigUint for &FieldElement {
    fn to_biguint(&self) -> Option<BigUint> {
        Some(self.value.clone())
    }
}

impl Clone for FieldElement {
    fn clone(&self) -> FieldElement {
        FieldElement::new(self.prime.clone(), self.value.clone())
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.prime == other.prime && self.value == other.value
    }
}

impl<Rhs> std::ops::Add<Rhs> for FieldElement
where
    Rhs: ToBigUint,
{
    type Output = FieldElement;
    fn add(self: Self, other: Rhs) -> FieldElement {
        FieldElement::new(self.value + other.to_biguint().unwrap(), self.prime)
    }
}

impl Eq for FieldElement {}

impl<Rhs> std::ops::AddAssign<Rhs> for FieldElement
where
    Rhs: ToBigUint,
{
    fn add_assign(&mut self, other: Rhs) {
        self.value += other.to_biguint().unwrap();
        self.value %= &self.prime;
    }
}

impl<Rhs> std::ops::Add<Rhs> for &FieldElement
where
    Rhs: ToBigUint,
{
    type Output = FieldElement;
    fn add(self: Self, other: Rhs) -> FieldElement {
        FieldElement::new(
            self.value.clone() + other.to_biguint().unwrap(),
            self.prime.clone(),
        )
    }
}

impl<Rhs> std::ops::Mul<Rhs> for FieldElement
where
    Rhs: ToBigUint,
{
    type Output = FieldElement;
    fn mul(self: Self, other: Rhs) -> FieldElement {
        FieldElement::new(
            self.value.clone() * other.to_biguint().unwrap(),
            self.prime.clone(),
        )
    }
}

impl<Rhs> std::ops::Mul<Rhs> for &FieldElement
where
    Rhs: ToBigUint,
{
    type Output = FieldElement;
    fn mul(self: Self, other: Rhs) -> FieldElement {
        FieldElement::new(
            self.value.clone() * other.to_biguint().unwrap(),
            self.prime.clone(),
        )
    }
}

impl<Rhs> std::ops::Sub<Rhs> for FieldElement
where
    Rhs: ToBigUint,
{
    type Output = FieldElement;
    fn sub(self: Self, other: Rhs) -> FieldElement {
        let mut multiplier = 1.to_biguint().unwrap();
        let other_val = other.to_biguint().unwrap();
        let mut self_val = self.value.clone();
        while self_val < other_val {
            self_val = (&self_val) + (self.get_prime() * &multiplier);
            multiplier *= 2.to_biguint().unwrap();
        }
        FieldElement::new(self_val - other_val, self.prime.clone())
    }
}

impl<Rhs> std::ops::Sub<Rhs> for &FieldElement
where
    Rhs: ToBigUint,
{
    type Output = FieldElement;
    fn sub(self: Self, other: Rhs) -> FieldElement {
        let mut multiplier = 1.to_biguint().unwrap();
        let other_val = other.to_biguint().unwrap();
        let mut self_val = self.value.clone();
        while self_val < other_val {
            self_val = (&self_val) + (self.get_prime() * &multiplier);
            multiplier *= 2.to_biguint().unwrap();
        }
        FieldElement::new(self_val - other_val, self.prime.clone())
    }
}

impl<Rhs> std::ops::SubAssign<Rhs> for FieldElement
where
    Rhs: ToBigUint,
{
    fn sub_assign(&mut self, other: Rhs) {
        let mut multiplier = 1.to_biguint().unwrap();
        let other_val = other.to_biguint().unwrap();
        let mut self_val = self.value.clone();
        while self_val < other_val {
            self_val = (&self_val) + (self.get_prime() * &multiplier);
            multiplier *= 2.to_biguint().unwrap();
        }
        self.value = self_val - other_val % &self.prime;
    }
}

impl<Rhs> std::ops::Div<Rhs> for FieldElement
where
    Rhs: ToBigUint,
{
    type Output = FieldElement;
    fn div(self: Self, other: Rhs) -> FieldElement {
        let two: BigUint = 2.to_biguint().unwrap();
        let inverse = FieldElement::new(other, self.prime.clone()).pow(&self.prime - two);
        &self * inverse
    }
}

impl<Rhs> std::ops::Div<Rhs> for &FieldElement
where
    Rhs: ToBigUint,
{
    type Output = FieldElement;
    fn div(self: Self, other: Rhs) -> FieldElement {
        let two: BigUint = 2.to_biguint().unwrap();
        let inverse = FieldElement::new(other, self.prime.clone()).pow(&self.prime - two);
        self * inverse
    }
}

impl<Rhs> std::ops::DivAssign<Rhs> for FieldElement
where
    Rhs: ToBigUint,
{
    fn div_assign(&mut self, other: Rhs) {
        let two: BigUint = 2.to_biguint().unwrap();
        let inverse = FieldElement::new(other, self.prime.clone()).pow(&self.prime - two);
        self.value = &self.value * inverse.value % &self.prime;
    }
}

impl<Rhs> std::ops::MulAssign<Rhs> for FieldElement
where
    Rhs: ToBigUint,
{
    fn mul_assign(&mut self, other: Rhs) {
        self.value = &self.value * other.to_biguint().unwrap();
        self.value %= &self.prime;
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0} (mod {1})", self.value, self.prime)
    }
}
