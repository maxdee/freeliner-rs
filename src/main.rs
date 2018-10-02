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



    // accessing a semgent is tedious
    // let ref group = _model.context.groups.get(0).unwrap();
    // let ref seg = group.segments.get(0);//.unwrap();
    // // at this point we have a option<SegmentType>
    // match seg {
    //     Some(SegmentType) => {
    //         let s = seg.unwrap();
    //         match s {
    //             SegmentType::straight(t) => {
    //                 let p1 = geom::point::pt2(t.a.x as f32, t.a.y as f32);
    //                 let p2 = geom::point::pt2(t.b.x as f32, t.b.y as f32);
    //
    //                 draw.line()
    //                    .start(p1)
    //                    .end(p2)
    //                    .thickness(3.0)
    //                    .caps_round()
    //                    .color(LIGHT_YELLOW);
    //             },
    //             _ => println!("other segment"),
    //         }
    //         // let () = frame.top_left();
    //
    //
    //     },
    //     None => println!("not a segment"),
    // }
}
