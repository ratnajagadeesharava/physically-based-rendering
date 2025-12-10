pub trait Tuple<T> {
    const nDimensions: usize;
    fn Add<T>(&self, other: &self);
}
