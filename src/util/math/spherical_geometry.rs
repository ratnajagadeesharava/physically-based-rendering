use std::{f32::consts::PI};
use num_traits::{clamp, ops::bytes::NumBytes};

use crate::util::{Float, vector::Vector3};
fn spherical_direction(sin_theta:Float,cos_theta:Float,phi:Float)->Vector3{
    Vector3::new(
        clamp(sin_theta, -1.0, 1.0)*phi.cos(),
        clamp(sin_theta, -1.0, 1.0)*phi.sin(),
        clamp(cos_theta, -1.0, 1.0)      
    )
}
fn spherical_triangle_area(a:Vector3,b:Vector3,c:Vector3)->Float{
    let numerator = a.dot(&b.cross(&c));    let denominator = 1.0+ a.dot(&b) +a.dot(&c)+b.dot(&c);
    let val = numerator/denominator;
    2.0*val.atan()
}

#[cfg(test)]
mod tests {
    use super::*;
   
}
