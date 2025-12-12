use crate::util::vector::{Point3, Vector3};

type Float = f32;

pub struct Ray {
    origin: Point3,
    direction: Vector3,
    time: Float,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3, time: Float) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }
    pub fn get(&self, t: Float) -> Point3 {
        &self.origin + &(&self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let time = 0.5;
        let ray = Ray::new(origin, direction, time);
        let point_at_t = ray.get(2.0);
        println!("Point at t=2.0: {:?}", point_at_t);
        let expected_point = Point3::new(9.0, 12.0, 15.0);
        // origin + direction * t = (1,2,3) + (4,5,6) * 2 = (1,2,3) + (8,10,12) = (9,12,15)
        assert_eq!(format!("{:?}", point_at_t), format!("{:?}", expected_point));
    }
}
