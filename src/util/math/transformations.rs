use crate::util::{math::SquareMatrix, rays::Ray};

pub struct Transform {
    m: SquareMatrix<4>,
    mInv: SquareMatrix<4>,
}

impl Transform {
    fn new(matrix: SquareMatrix<4>, inverseMatrix: SquareMatrix<4>) -> Self {
        Self {
            m: matrix,
            mInv: inverseMatrix,
        }
    }
}
