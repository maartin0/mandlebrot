use std::ops::{Index, Mul, MulAssign};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Matrix3(pub [f32; 9]);

impl Matrix3 {
    pub fn scale(x: f32, y: f32) -> Self {
        Self([x, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 1.0])
    }

    pub fn identity() -> Self {
        Self([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
    }

    pub fn translate(x: f32, y: f32) -> Self {
        Self([1.0, 0.0, x, 0.0, 1.0, y, 0.0, 0.0, 1.0])
    }
}

impl From<[f32; 9]> for Matrix3 {
    fn from(value: [f32; 9]) -> Self {
        Self(value)
    }
}

impl From<Matrix3> for [f32; 9] {
    fn from(value: Matrix3) -> Self {
        value.0
    }
}

impl From<&[f32; 9]> for Matrix3 {
    fn from(value: &[f32; 9]) -> Self {
        Self(*value)
    }
}

impl<'a> From<&'a Matrix3> for &'a [f32; 9] {
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
            self[0] * rhs[0] + self[1] * rhs[3] + self[2] * rhs[6],
            self[0] * rhs[1] + self[1] * rhs[4] + self[2] * rhs[7],
            self[0] * rhs[2] + self[1] * rhs[5] + self[2] * rhs[8],
            self[3] * rhs[0] + self[4] * rhs[3] + self[5] * rhs[6],
            self[3] * rhs[1] + self[4] * rhs[4] + self[5] * rhs[7],
            self[3] * rhs[2] + self[4] * rhs[5] + self[5] * rhs[8],
            self[6] * rhs[0] + self[7] * rhs[3] + self[8] * rhs[6],
            self[6] * rhs[1] + self[7] * rhs[4] + self[8] * rhs[7],
            self[6] * rhs[2] + self[7] * rhs[5] + self[8] * rhs[8],
        ])
    }
}

impl MulAssign for Matrix3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul for &Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: Self) -> Self::Output {
        *self * *rhs
    }
}

impl Mul<&Matrix3> for Matrix3 {
    type Output = Self;

    fn mul(self, rhs: &Matrix3) -> Self::Output {
        self * *rhs
    }
}

impl Mul<Matrix3> for &Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        *self * rhs
    }
}

impl MulAssign<&Matrix3> for Matrix3 {
    fn mul_assign(&mut self, rhs: &Matrix3) {
        *self *= *rhs;
    }
}

impl Index<usize> for Matrix3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<usize> for &Matrix3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<(usize, usize)> for Matrix3 {
    type Output = f32;
    fn index(&self, (column, row): (usize, usize)) -> &Self::Output {
        &self.0[row * 3 + column]
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix3;

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
            Matrix3([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0])
                * Matrix3([9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]),
            Matrix3([30.0, 24.0, 18.0, 84.0, 69.0, 54.0, 138.0, 114.0, 90.0])
        )
    }

    #[test]
    fn dual_index() {
        assert_eq!(
            Matrix3([0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])[(1, 0)],
            1.0
        );
        assert_eq!(
            Matrix3([0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0])[(2, 1)],
            1.0
        );
    }
}
