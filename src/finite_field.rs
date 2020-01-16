use num_bigint::{BigUint, ToBigInt, ToBigUint};
// use num_integer::Integer;
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
    pub fn add_int_ref(&mut self, other: &BigUint) {
        self.value += other;
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
        // Set all types to bigint
        let one = 1.to_bigint().unwrap();
        let big_exponent = exp.to_bigint().unwrap();
        let prime_less_one = self.prime.to_bigint().unwrap() - &one;

        // Reduce exponent to its smallest positive equivalent using bigint's builtin modpow, which uses true mod not %
        let reduced_expt = big_exponent.modpow(&one, &prime_less_one);
        // Compute self ** reduced_expt
        let new_value = self
            .value
            .modpow(&reduced_expt.to_biguint().unwrap(), &self.prime);
        FieldElement::new(new_value, self.prime.clone())
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
impl Eq for FieldElement {}

impl<Rhs> std::ops::Add<Rhs> for FieldElement
where
    Rhs: ToBigUint,
{
    type Output = FieldElement;
    fn add(self: Self, other: Rhs) -> FieldElement {
        FieldElement::new(self.value + other.to_biguint().unwrap(), self.prime)
    }
}

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
        let other = other.to_biguint().unwrap() % &self.prime;
        if self.value < other {
            return FieldElement::new((self.value + &self.prime) - other, self.prime.clone());
        }
        FieldElement::new(self.value - other, self.prime.clone())
    }
}

impl<Rhs> std::ops::Sub<Rhs> for &FieldElement
where
    Rhs: ToBigUint,
{
    type Output = FieldElement;
    fn sub(self: Self, other: Rhs) -> FieldElement {
        let other = other.to_biguint().unwrap() % &self.prime;
        if self.value < other {
            return FieldElement::new((&self.value + &self.prime) - other, self.prime.clone());
        }
        FieldElement::new(&self.value - other, self.prime.clone())
    }
}

impl<Rhs> std::ops::SubAssign<Rhs> for FieldElement
where
    Rhs: ToBigUint,
{
    fn sub_assign(&mut self, other: Rhs) {
        let other = other.to_biguint().unwrap() % &self.prime;
        if self.value < other {
            self.value = &self.value + &self.prime - other;
        } else {
            self.value -= other;
        }
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
