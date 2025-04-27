use std::ops::{Index, Mul, MulAssign};

use crate::arbitrary_num::ArbitaryNum;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Matrix3(pub [ArbitaryNum; 9]);

impl Matrix3 {
    pub fn scale(x: ArbitaryNum, y: ArbitaryNum) -> Self {
        Self([
            x,
            ArbitaryNum::zero(),
            ArbitaryNum::zero(),
            ArbitaryNum::zero(),
            y,
            ArbitaryNum::zero(),
            ArbitaryNum::zero(),
            ArbitaryNum::zero(),
            ArbitaryNum::one(),
        ])
    }

    pub fn identity() -> Self {
        Self([
            ArbitaryNum::one(),
            ArbitaryNum::zero(),
            ArbitaryNum::zero(),
            ArbitaryNum::zero(),
            ArbitaryNum::one(),
            ArbitaryNum::zero(),
            ArbitaryNum::zero(),
            ArbitaryNum::zero(),
            ArbitaryNum::one(),
        ])
    }

    pub fn translate(x: ArbitaryNum, y: ArbitaryNum) -> Self {
        Self([
            ArbitaryNum::one(),
            ArbitaryNum::zero(),
            x,
            ArbitaryNum::zero(),
            ArbitaryNum::one(),
            y,
            ArbitaryNum::zero(),
            ArbitaryNum::zero(),
            ArbitaryNum::one(),
        ])
    }
}

impl From<[ArbitaryNum; 9]> for Matrix3 {
    fn from(value: [ArbitaryNum; 9]) -> Self {
        Self(value)
    }
}

impl From<Matrix3> for [ArbitaryNum; 9] {
    fn from(value: Matrix3) -> Self {
        value.0
    }
}

impl<'a> From<&'a Matrix3> for &'a [ArbitaryNum; 9] {
    fn from(value: &'a Matrix3) -> Self {
        &value.0
    }
}

impl Default for Matrix3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul for Matrix3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self([
            self[0].clone() * rhs[0].clone()
                + self[1].clone() * rhs[3].clone()
                + self[2].clone() * rhs[6].clone(),
            self[0].clone() * rhs[1].clone()
                + self[1].clone() * rhs[4].clone()
                + self[2].clone() * rhs[7].clone(),
            self[0].clone() * rhs[2].clone()
                + self[1].clone() * rhs[5].clone()
                + self[2].clone() * rhs[8].clone(),
            self[3].clone() * rhs[0].clone()
                + self[4].clone() * rhs[3].clone()
                + self[5].clone() * rhs[6].clone(),
            self[3].clone() * rhs[1].clone()
                + self[4].clone() * rhs[4].clone()
                + self[5].clone() * rhs[7].clone(),
            self[3].clone() * rhs[2].clone()
                + self[4].clone() * rhs[5].clone()
                + self[5].clone() * rhs[8].clone(),
            self[6].clone() * rhs[0].clone()
                + self[7].clone() * rhs[3].clone()
                + self[8].clone() * rhs[6].clone(),
            self[6].clone() * rhs[1].clone()
                + self[7].clone() * rhs[4].clone()
                + self[8].clone() * rhs[7].clone(),
            self[6].clone() * rhs[2].clone()
                + self[7].clone() * rhs[5].clone()
                + self[8].clone() * rhs[8].clone(),
        ])
    }
}

impl MulAssign for Matrix3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl Mul for &Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: Self) -> Self::Output {
        self.clone() * rhs.clone()
    }
}

impl Mul<&Matrix3> for Matrix3 {
    type Output = Self;

    fn mul(self, rhs: &Matrix3) -> Self::Output {
        self.clone() * rhs.clone()
    }
}

impl Mul<Matrix3> for &Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        self.clone() * rhs
    }
}

impl MulAssign<&Matrix3> for Matrix3 {
    fn mul_assign(&mut self, rhs: &Matrix3) {
        *self *= rhs.clone();
    }
}

impl Index<usize> for Matrix3 {
    type Output = ArbitaryNum;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<usize> for &Matrix3 {
    type Output = ArbitaryNum;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<(usize, usize)> for Matrix3 {
    type Output = ArbitaryNum;
    fn index(&self, (column, row): (usize, usize)) -> &Self::Output {
        &self.0[row * 3 + column]
    }
}

impl Mul<(ArbitaryNum, ArbitaryNum, ArbitaryNum)> for Matrix3 {
    type Output = (ArbitaryNum, ArbitaryNum, ArbitaryNum);
    fn mul(self, rhs: (ArbitaryNum, ArbitaryNum, ArbitaryNum)) -> Self::Output {
        (
            self[0].clone() * rhs.0.clone()
                + self[1].clone() * rhs.1.clone()
                + self[2].clone() * rhs.2.clone(),
            self[3].clone() * rhs.0.clone()
                + self[4].clone() * rhs.1.clone()
                + self[5].clone() * rhs.2.clone(),
            self[6].clone() * rhs.0.clone()
                + self[7].clone() * rhs.1.clone()
                + self[8].clone() * rhs.2.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_mul() {
        assert_eq!(
            Matrix3::identity() * Matrix3::identity(),
            Matrix3::identity()
        )
    }

    #[test]
    fn mul() {
        assert_eq!(
            Matrix3([
                1.into(),
                2.into(),
                3.into(),
                4.into(),
                5.into(),
                6.into(),
                7.into(),
                8.into(),
                9.into()
            ]) * Matrix3([
                9.into(),
                8.into(),
                7.into(),
                6.into(),
                5.into(),
                4.into(),
                3.into(),
                2.into(),
                1.into(),
            ]),
            Matrix3([
                30.into(),
                24.into(),
                18.into(),
                84.into(),
                69.into(),
                54.into(),
                138.into(),
                114.into(),
                90.into()
            ])
        )
    }

    #[test]
    fn dual_index() {
        assert_eq!(
            Matrix3([
                ArbitaryNum::zero(),
                ArbitaryNum::one(),
                ArbitaryNum::zero(),
                ArbitaryNum::zero(),
                ArbitaryNum::zero(),
                ArbitaryNum::zero(),
                ArbitaryNum::zero(),
                ArbitaryNum::zero(),
                ArbitaryNum::zero()
            ])[(1, 0)],
            ArbitaryNum::one()
        );
        assert_eq!(
            Matrix3([
                ArbitaryNum::zero(),
                ArbitaryNum::zero(),
                ArbitaryNum::zero(),
                ArbitaryNum::zero(),
                ArbitaryNum::zero(),
                ArbitaryNum::one(),
                ArbitaryNum::zero(),
                ArbitaryNum::zero(),
                ArbitaryNum::zero()
            ])[(2, 1)],
            ArbitaryNum::one()
        );
    }
}
