extern crate freeliner;
use freeliner::geometry::*;

fn main(){
    let mut context = Context{groups : Vec::<SegmentGroup>::new(),};
    context.start();
    println!("welcome to freeliner-rs version {}", freeliner::get_version());
    // context.
    let point_a = Point::new(0.0, 2.0, 0.0);
    let point_b = Point::new(3.0, 2.0, 42.0);
    println!("{:?}", Point::lerp(&point_a, &point_b, 0.5));
}
