use std::ops::{Add, Index, Mul, Sub};
use std::{convert::identity, vec};

use crate::util::types::{Float, Int};
#[derive(Debug)]
pub struct SquareMatrix<const N: usize> {
    matrix: [[Float; N]; N],
}

impl<const N: usize> SquareMatrix<N> {
    pub fn new() -> Self {
        SquareMatrix::<N>::identity()
    }
    pub fn fill(data: Vec<Float>) -> Self {
        if data.len() < N * N {
            panic!("input array is less than size of the matrix")
        }
        let mut mat = [[0.0; N]; N];
        let mut count = 0;
        for i in 0..N {
            for j in 0..N {
                mat[i][j] = data[count];
                count += 1;
            }
        }
        Self { matrix: mat }
    }
    pub fn identity() -> Self {
        let mut mat = [[0.0; N]; N];
        for i in 0..N {
            mat[i][i] = 1.0;
        }
        Self { matrix: mat }
    }
    pub fn zero() -> Self {
        let mut mat = [[0.0; N]; N];
        Self { matrix: mat }
    }
}

impl<const N: usize> Add<&SquareMatrix<N>> for &SquareMatrix<N> {
    type Output = SquareMatrix<N>;
    fn add(self, rhs: &SquareMatrix<N>) -> Self::Output {
        let mut result = SquareMatrix::<N>::zero();
        for i in 0..N {
            for j in 0..N {
                result.matrix[i][j] = self.matrix[i][j] + rhs.matrix[i][j];
            }
        }
        result
    }
}
impl<const N: usize> Sub<&SquareMatrix<N>> for &SquareMatrix<N> {
    type Output = SquareMatrix<N>;
    fn sub(self, rhs: &SquareMatrix<N>) -> Self::Output {
        let mut result = SquareMatrix::<N>::zero();
        for i in 0..N {
            for j in 0..N {
                result.matrix[i][j] = self.matrix[i][j] - rhs.matrix[i][j];
            }
        }
        result
    }
}
impl<const N: usize> PartialEq for SquareMatrix<N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            for j in 0..N {
                if self.matrix[i][j] != other.matrix[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::util::math::SquareMatrix;
    #[test]
    fn add_2X2_matrix() {
        let mat = SquareMatrix::<2>::fill(vec![0.0, 1.0, 2.0, 3.0]);
        let mat2 = SquareMatrix::<2>::fill(vec![2.0, 3.0, 5.0, 2.0]);
        let result = &mat + &mat2;
        let expected = SquareMatrix::<2>::fill(vec![2.0, 4.0, 7.0, 5.0]);
        assert_eq!(result, expected);
    }
    #[test]
    fn sub_2X2_matrix() {
        let mat = SquareMatrix::<2>::fill(vec![0.0, 1.0, 2.0, 3.0]);
        let mat2 = SquareMatrix::<2>::fill(vec![2.0, 3.0, 5.0, 2.0]);
        let result = &mat - &mat2;
        let expected = SquareMatrix::<2>::fill(vec![-2.0, -2.0, -3.0, 1.0]);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_4X4_identity() {
        let mat = SquareMatrix::<4>::identity();
        let mut flag = true;
        for i in 0..4 {
            flag = flag && mat.matrix[i][i] == 1.0;
        }
        assert_eq!(flag, true);
    }
}
