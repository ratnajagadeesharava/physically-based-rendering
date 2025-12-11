use std::ops::{Add, Index, Mul, Sub};
#[derive(Debug)]
pub struct Tuple3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Tuple3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Tuple3<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z + other.z
    }
}
impl<T> PartialEq for Tuple3<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
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

impl<T> Mul<Tuple3<T>> for Tuple3<T>
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

impl<T> Mul<T> for Tuple3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
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

#[test]
fn add_tupple_3() {
    let mut tup1 = Tuple3::new(1, 2, 4);
    let mut tup2 = Tuple3::new(2, 3, 4);
    tup1 = tup2 + tup1;
    let result = Tuple3::new(3, 5, 8);
    assert_eq!(result, tup1);
}
#[test]
fn mul_tupple_3() {
    let mut tup1 = Tuple3::new(1, 2, 4);
    let mut tup2 = Tuple3::new(2, 3, 4);
    tup1 = tup2 * tup1;
    let result = Tuple3::new(2, 6, 16);
    assert_eq!(result, tup1);
}
#[test]
fn sub_tupple_3() {
    let mut tup1 = Tuple3::new(1, 2, 4);
    let mut tup2 = Tuple3::new(2, 3, 4);
    tup1 = tup1 - tup2;
    let result = Tuple3::new(-1, -1, 0);
    assert_eq!(result, tup1);
}
#[test]
fn check_dot() {
    let tup1 = Tuple3::new(1, 2, 4);
    let tup2 = Tuple3::new(2, 3, 4);
    let m = tup1.dot(&tup2);
    assert_eq!(16, m);
}

#[test]
fn check_scaler_mul() {
    let mut tup1 = Tuple3::new(1, 2, 4);
    let result = Tuple3::new(4, 8, 16);
    tup1 = tup1 * 4;
    assert_eq!(tup1, result);
}
