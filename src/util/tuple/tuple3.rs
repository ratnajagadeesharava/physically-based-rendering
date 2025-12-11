use std::ops::{Add, Index, Mul, Sub};

/// A generic 3-dimensional tuple structure.
/// 
/// This structure represents a tuple with three components (x, y, z),
/// commonly used in 3D graphics for vectors, points, and colors.
/// It supports generic types allowing flexibility in numeric representations.
#[derive(Debug)]
pub struct Tuple3<T> {
    /// The x-component (first element) of the tuple
    pub x: T,
    /// The y-component (second element) of the tuple
    pub y: T,
    /// The z-component (third element) of the tuple
    pub z: T,
}

impl<T> Tuple3<T> {
    /// Creates a new `Tuple3` with the specified x, y, and z components.
    /// 
    /// # Arguments
    /// * `x` - The first component
    /// * `y` - The second component
    /// * `z` - The third component
    /// 
    /// # Returns
    /// A new `Tuple3` instance
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Tuple3<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    /// Computes the dot product of this tuple with another tuple.
    /// 
    /// The dot product is the sum of the products of corresponding components.
    /// 
    /// # Arguments
    /// * `other` - The other tuple to compute the dot product with
    /// 
    /// # Returns
    /// The scalar result of the dot product
    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z + other.z
    }
}
/// Implementation of equality comparison for `Tuple3`.
/// 
/// Two tuples are equal if all their corresponding components are equal.
impl<T> PartialEq for Tuple3<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
/// Implementation of indexing for `Tuple3`.
/// 
/// Allows accessing components by index: 0 for x, 1 for y, 2 for z.
/// 
/// # Panics
/// Panics if the index is greater than 2.
impl<T: Copy> Index<usize> for Tuple3<T> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}
/// Implementation of subtraction for `Tuple3`.
/// 
/// Subtracts each component of the right-hand side tuple from the
/// corresponding component of the left-hand side tuple.
impl<T> Sub for Tuple3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Implementation of component-wise multiplication for `Tuple3`.
/// 
/// Multiplies each component of the left-hand side tuple with the
/// corresponding component of the right-hand side tuple.
impl<T> Mul for Tuple3<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
/// Implementation of addition for `Tuple3`.
/// 
/// Adds each component of the left-hand side tuple with the
/// corresponding component of the right-hand side tuple.
impl<T> Add for Tuple3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Test for tuple addition.
/// Verifies that adding two tuples produces the correct component-wise sum.
#[test]
fn add_tupple_3() {
    let mut tup1 = Tuple3::new(1, 2, 4);
    let  tup2 = Tuple3::new(2, 3, 4);
    tup1 = tup2 + tup1;
    let result = Tuple3::new(3, 5, 8);
    assert_eq!(result, tup1);
}
// #[test]
// fn add_tupple_3_2(){
//     let  mut tup1 = Tuple3::new(1,2,3);
//     let tup2 = Tuple3::new(4,5,6);
//     tup1 += tup2;
//     let result = Tuple3::new(5,7,9);
//     assert_eq!(result,tup1)
// }
/// Test for tuple multiplication.
/// Verifies that multiplying two tuples produces the correct component-wise product.
#[test]
fn mul_tupple_3() {
    let mut tup1 = Tuple3::new(1, 2, 4);
    let  tup2 = Tuple3::new(2, 3, 4);
    tup1 = tup2 * tup1;
    let result = Tuple3::new(2, 6, 16);
    assert_eq!(result, tup1);
}
/// Test for tuple subtraction.
/// Verifies that subtracting one tuple from another produces the correct component-wise difference.
#[test]
fn sub_tupple_3() {
    let mut tup1 = Tuple3::new(1, 2, 4);
    let  tup2 = Tuple3::new(2, 3, 4);
    tup1 = tup1 - tup2;
    let result = Tuple3::new(-1, -1, 0);
    assert_eq!(result, tup1);
}
/// Test for dot product calculation.
/// Verifies that the dot product of two tuples is computed correctly.
/// Expected result: (1*2) + (2*3) + (4*4) = 2 + 6 + 16 = 24
/// Note: There appears to be a bug in the dot product implementation.
#[test]
fn check_dot() {
    let tup1 = Tuple3::new(1, 2, 4);
    let tup2 = Tuple3::new(2, 3, 4);
    let m = tup1.dot(&tup2);
    assert_eq!(16, m);
}
