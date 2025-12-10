use std::ops::{Add, Index, Mul};
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
