use std::ops::{Add, Mul, Sub};
#[derive(Debug)]
pub struct Tuple2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Tuple2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}


impl<T> Tuple2<T> where T:Add<Output=T>+Mul<Output=T>+Copy{
    pub fn dot(&self,other:&Self)->T{
        self.x*other.x+self.y*other.y
    }
}
impl<T> PartialEq for Tuple2<T> where T:PartialEq{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl <T> Add for Tuple2<T> where T:Add<Output=T>{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T> Sub for Tuple2<T> where T:Sub<Output=T>{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul for Tuple2<T> where T:Mul<Output=T>+Copy{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}


#[test]
fn test_tuple2_dot() {
    let t1 = Tuple2::new(1, 2);
    let t2 = Tuple2::new(3, 4);
    let result = t1.dot(&t2);

    assert_eq!(result, 11);
}

#[test]
fn test_tuple2_add() {
    let t1 = Tuple2::new(1, 2);
    let t2 = Tuple2::new(3, 4);
    let result = t1 + t2;

    assert_eq!(result, Tuple2::new(4, 6));
}

#[test]
fn test_tuple2_sub() {
    let t1 = Tuple2::new(1, 2);
    let t2 = Tuple2::new(3, 4);
    let result = t1 - t2;

    assert_eq!(result, Tuple2::new(-2, -2));
}

#[test]
fn test_tuple2_mul() {
    let t1 = Tuple2::new(1, 2);
    let t2 = Tuple2::new(3, 4);
    let result = t1 * t2;

    assert_eq!(result, Tuple2::new(3, 8));
}
