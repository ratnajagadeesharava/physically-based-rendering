use crate::util::tuple::{Tuple2, Tuple3};
use crate::util::vector::{Normal3, Point3, Vector3};
use crate::util::bounds::Bounds3;

mod types;
pub use types::*;

mod tuple;
pub mod vector;
pub mod rays;

pub mod bounds;
pub mod math;
