use crate::util::{math::SquareMatrix, rays::Ray, vector::Vector3};

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

    fn transform(&self, v: Vector3) -> Vector3 {
        Vector3::new(
            self.m.matrix[0][0] * v.get_x()
                + self.m.matrix[0][1] * v.get_y()
                + self.m.matrix[0][2] * v.get_z()
                + self.m.matrix[0][3],
            self.m.matrix[1][0] * v.get_x()
                + self.m.matrix[1][1] * v.get_y()
                + self.m.matrix[1][2] * v.get_z()
                + self.m.matrix[1][3],
            self.m.matrix[2][0] * v.get_x()
                + self.m.matrix[2][1] * v.get_y()
                + self.m.matrix[2][2] * v.get_z()
                + self.m.matrix[2][3],
        )
    }
}
