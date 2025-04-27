use std::ops::{Add, Div, Mul, Neg, Sub};

use num::{rational::Ratio, BigInt, BigRational, FromPrimitive, One, Signed, ToPrimitive, Zero};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArbitaryNum(BigRational);

impl ArbitaryNum {
    pub fn zero() -> Self {
        Self(BigRational::zero())
    }

    pub fn half() -> Self {
        Self(Ratio::new_raw(BigInt::one(), BigInt::from_u16(2).unwrap()))
    }

    pub fn one() -> Self {
        Self(BigRational::one())
    }

    pub fn two() -> Self {
        Self(Ratio::new_raw(BigInt::from_u16(2).unwrap(), BigInt::one()))
    }
}

impl Add for ArbitaryNum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for ArbitaryNum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for ArbitaryNum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for ArbitaryNum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Neg for ArbitaryNum {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl From<ArbitaryNum> for f32 {
    fn from(value: ArbitaryNum) -> Self {
        value.0.to_f32().unwrap()
    }
}

impl From<f32> for ArbitaryNum {
    fn from(value: f32) -> Self {
        Self(BigRational::from_float(value).unwrap())
    }
}

impl From<f64> for ArbitaryNum {
    fn from(value: f64) -> Self {
        Self(BigRational::from_float(value).unwrap())
    }
}

impl From<i32> for ArbitaryNum {
    fn from(value: i32) -> Self {
        (value as f64).into()
    }
}

impl From<u32> for ArbitaryNum {
    fn from(value: u32) -> Self {
        Self(Ratio::new_raw(
            BigInt::from_u32(value).unwrap(),
            BigInt::one(),
        ))
    }
}

impl ArbitaryNum {
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
}
