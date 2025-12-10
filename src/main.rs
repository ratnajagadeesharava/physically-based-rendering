mod util;
use crate::util::tuple::Tuple2;
use crate::util::tuple::Tuple3;
fn main() {
    let t = Tuple3::<f64>::new(2.0, 3.0, 4.0);
    println!("x: {}, y: {}, z:{}", t.x, t.y, t.z);
}
