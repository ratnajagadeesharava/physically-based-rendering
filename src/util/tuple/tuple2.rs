pub struct Tuple2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Tuple2<T> {
    pub const nDimension: usize = 2;
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
