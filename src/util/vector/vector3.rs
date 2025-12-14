use std::ops::{Add, Index, Mul, Sub};
type Float = f32;
#[derive(Debug)]
pub struct Vector3 {
    x: Float,
    y: Float,
    z: Float,
}
impl From<(Float, Float, Float)> for Vector3 {
    fn from(value: (Float, Float, Float)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl Vector3 {
    pub fn get_x(&self) -> Float {
        self.x
    }
    pub fn get_y(&self) -> Float {
        self.y
    }
    pub fn get_z(&self) -> Float {
        self.z
    }
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x: x, y: y, z: z }
    }
    pub fn has_nan(&self) -> bool {
        self.x.is_nan() && self.y.is_nan() && self.z.is_nan()
    }
    pub fn dot(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn length(&self) -> Float {
        let x = self.x * self.x + self.y * self.y + self.z * self.z;
        x.sqrt()
    }

    #[inline]
    pub fn distance(v1: &Vector3, v2: &Vector3) -> Float {
        (v1 - v2).length()
    }
    pub fn normalize(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn angle_between(v1: &Vector3, v2: &Vector3) -> Float {
        let dot_product = v1.dot(v2);
        let lengths_product = v1.length() * v2.length();
        let cos_theta = dot_product / lengths_product;
        let angle = cos_theta.acos();
        angle
    }
}

// Operators Overloading
//
impl Clone for Vector3 {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
impl Index<usize> for Vector3 {
    type Output = Float;
    fn index(&self, i: usize) -> &Self::Output {
        if i == 0 {
            &self.x
        } else if i == 1 {
            &self.y
        } else if i == 2 {
            &self.z
        } else {
            panic!("index  out of bounds")
        }
    }
}

impl Sub<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Add<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Mul<Vector3> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Float> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Float) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<Float> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Float) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

/// Test for tuple addition.
/// Verifies that adding two tuples produces the correct component-wise sum.
#[test]
fn add_tupple_3() {
    let mut tup1 = Vector3::new(1.0, 2.0, 4.0);
    let tup2 = Vector3::new(2.0, 3.0, 4.0);
    tup1 = tup2 + tup1;
    let result = Vector3::new(3.0, 5.0, 8.0);
    assert_eq!(result, tup1);
}
// #[test]
// fn add_tupple_3_2(){
//     let  mut tup1 = Vector3::new(1,2,3);
//     let tup2 = Vector3::new(4,5,6);
//     tup1 += tup2;
//     let result = Vector3::new(5,7,9);
//     assert_eq!(result,tup1)
// }
/// Test for tuple multiplication.
/// Verifies that multiplying two tuples produces the correct component-wise product.
#[test]
fn mul_tupple_3() {
    let mut tup1 = Vector3::new(1.0, 2.0, 4.0);
    let tup2 = Vector3::new(2.0, 3.0, 4.0);
    tup1 = tup2 * tup1;
    let result = Vector3::new(2.0, 6.0, 16.0);
    assert_eq!(result, tup1);
}
/// Test for tuple subtraction.
/// Verifies that subtracting one tuple from another produces the correct component-wise difference.
#[test]
fn sub_tupple_3() {
    let tup1 = Vector3::new(1.0, 2.0, 4.0);
    let tup2 = Vector3::new(2.0, 3.0, 4.0);
    let res = &tup1 - &tup2;
    let result = Vector3::new(-1.0, -1.0, 0.0);
    assert_eq!(result, res);
}

/// Test for dot product calculation.
/// Verifies that the dot product of two tuples is computed correctly.
/// Expected result: (1*2) + (2*3) + (4*4) = 2 + 6 + 16 = 24
/// Note: There appears to be a bug in the dot product implementation.
#[test]
fn check_dot() {
    let tup1 = Vector3::new(1.0, 2.0, 4.0);
    let tup2 = Vector3::new(2.0, 3.0, 4.0);
    let m = tup1.dot(&tup2);
    println!("Dot Product Result: {}", m);
    assert_eq!(24.0, m);
}

#[test]
fn check_scaler_mul() {
    let mut tup1 = Vector3::new(1.0, 2.0, 4.0);
    let result = Vector3::new(4.0, 8.0, 16.0);
    tup1 = &tup1 * 4.0;

    assert_eq!(tup1, result);
}
pub type Point3 = Vector3;
pub type Normal3 = Vector3;
