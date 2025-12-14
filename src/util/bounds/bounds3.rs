type Float = f32;
use crate::util::vector::{Point3, Vector3};
pub struct Bounds3 {
    pMin: Point3,
    pMax: Point3,
}

impl Bounds3 {
    fn new() -> Self {
        let minNum = Float::MIN;
        let maxNum = Float::MAX;
        Self {
            pMin: Point3::new(maxNum, maxNum, maxNum),
            pMax: Point3::new(minNum, minNum, minNum),
        }
    }
    fn union_point(&self, point: &Point3) -> Bounds3 {
        let pMin = Point3::new(
            self.pMin.get_x().min(point.get_x()),
            self.pMin.get_y().min(point.get_y()),
            self.pMin.get_z().min(point.get_z()),
        );
        let pMax = Point3::new(
            self.pMax.get_x().max(point.get_x()),
            self.pMax.get_y().max(point.get_y()),
            self.pMax.get_z().max(point.get_z()),
        );
        Bounds3 { pMin, pMax }
    }
    fn union_bounds(&self, bounds: &Bounds3) -> Bounds3 {
        let mut bounds = self.union_point(&bounds.pMin);
        bounds = bounds.union_point(&bounds.pMax);
        bounds
    }
    #[inline]
    fn overlaps(b1: Bounds3, b2: Bounds3) -> bool {
        let x = (b1.pMax.get_x() >= b2.pMin.get_x()) && (b1.pMin.get_x() <= b2.pMax.get_x());
        let y = (b1.pMax.get_y() >= b2.pMin.get_y()) && (b1.pMin.get_y() <= b2.pMax.get_y());
        let z = (b1.pMax.get_z() >= b2.pMin.get_z()) && (b1.pMin.get_z() <= b2.pMax.get_z());
        x && y && z
    }
    #[inline]
    fn is_point_inside(&self, point: &Point3) -> bool {
        (point.get_x() >= self.pMin.get_x()
            && point.get_x() <= self.pMax.get_x()
            && point.get_y() >= self.pMin.get_y()
            && point.get_y() <= self.pMax.get_y()
            && point.get_z() >= self.pMin.get_z()
            && point.get_z() <= self.pMax.get_z())
    }
    fn diagonal(&self) -> Vector3 {
        &self.pMax - &self.pMin
    }
    fn surface_area(&self) -> Float {
        let dg = self.diagonal();
        2.0 * (dg.get_x() * dg.get_y() + dg.get_y() * dg.get_z() + dg.get_z() * dg.get_x())
    }
    fn expand(b1: Bounds3, delta: Float) -> Bounds3 {
        let pMin = Vector3::new(
            b1.pMin.get_x() - delta,
            b1.pMin.get_y() - delta,
            b1.pMin.get_z() - delta,
        );
        let pMax = Vector3::new(
            b1.pMin.get_x() + delta,
            b1.pMin.get_y() + delta,
            b1.pMin.get_z() + delta,
        );
        Bounds3 { pMin, pMax }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bounds3() {
        let b = Bounds3::new();
        assert_eq!(b.pMin.get_x(), Float::MAX);
        assert_eq!(b.pMin.get_y(), Float::MAX);
        assert_eq!(b.pMin.get_z(), Float::MAX);
        assert_eq!(b.pMax.get_x(), Float::MIN);
        assert_eq!(b.pMax.get_y(), Float::MIN);
        assert_eq!(b.pMax.get_z(), Float::MIN);
    }

    #[test]
    fn test_union_point() {
        let b = Bounds3::new();
        let p = Point3::new(1.0, 2.0, 3.0);
        let b2 = b.union_point(&p);
        assert_eq!(b2.pMin.get_x(), 1.0);
        assert_eq!(b2.pMin.get_y(), 2.0);
        assert_eq!(b2.pMin.get_z(), 3.0);
        assert_eq!(b2.pMax.get_x(), 1.0);
        assert_eq!(b2.pMax.get_y(), 2.0);
        assert_eq!(b2.pMax.get_z(), 3.0);
    }

    #[test]
    fn test_union_bounds() {
        let b1 = Bounds3::new().union_point(&Point3::new(0.0, 0.0, 0.0));
        let b2 = Bounds3::new().union_point(&Point3::new(1.0, 2.0, 3.0));
        let b3 = b1.union_bounds(&b2);
        assert_eq!(b3.pMin.get_x(), 0.0);
        assert_eq!(b3.pMin.get_y(), 0.0);
        assert_eq!(b3.pMin.get_z(), 0.0);
        assert_eq!(b3.pMax.get_x(), 1.0);
        assert_eq!(b3.pMax.get_y(), 2.0);
        assert_eq!(b3.pMax.get_z(), 3.0);
    }

    #[test]
    fn test_overlaps_true() {
        let b1 = Bounds3::new()
            .union_point(&Point3::new(0.0, 0.0, 0.0))
            .union_point(&Point3::new(2.0, 2.0, 2.0));
        let b2 = Bounds3::new()
            .union_point(&Point3::new(1.0, 1.0, 1.0))
            .union_point(&Point3::new(3.0, 3.0, 3.0));
        assert!(Bounds3::overlaps(b1, b2));
    }

    #[test]
    fn test_overlaps_false() {
        let b1 = Bounds3::new()
            .union_point(&Point3::new(0.0, 0.0, 0.0))
            .union_point(&Point3::new(1.0, 1.0, 1.0));
        let b2 = Bounds3::new()
            .union_point(&Point3::new(2.0, 2.0, 2.0))
            .union_point(&Point3::new(3.0, 3.0, 3.0));
        assert!(!Bounds3::overlaps(b1, b2));
    }
}
