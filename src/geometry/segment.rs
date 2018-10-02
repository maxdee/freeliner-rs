use super::point::*;

pub trait SegStuff { // or get_pos trait???
    fn get_pos(&self, u:f64) -> Point;
    fn length(&self) -> f64;
}

#[derive(Debug)]
pub enum SegmentType {
    straight(StraightSegment),
    bezier(BezierSegment),
    nested,
}

// impl SegmentType {
//     fn get_pos(&self, u: f64) -> Point{}
// }

#[derive(Debug)]
pub struct StraightSegment {
    pub a: Point,
    pub b: Point,
}

impl SegStuff for StraightSegment {
    fn get_pos(&self, unit:f64) -> Point {
        Point::lerp(&self.a, &self.b, unit)
    }
    fn length(&self) -> f64 {
        self.a.dist(&self.b)
    }
}

pub fn interpolator<T: SegStuff>(seg: T, u: f64) -> Point{
    // do different stuff for different type of segments??
    // match types?
    Point::new(0.0, 0.0, 0.0)
}

/*
pub fn some_func<T, U>(seg: T, event: U) -> Point
    where T: SegStugg + Debug,
          U: Event + Debug
{

}
*/


// enum BezierTypes {
//     cubic,
//
// }

// types of segments


#[derive(Debug)]
pub struct BezierSegment {
    a: Point,
    b: Point,
    ah: Point,
    bh: Point,
}

// #[derive(Debug)]
pub struct NestedSegment {
    segments: Vec<SegmentType>,
}

// pub fn scale(seg_type: SegmentTypes){
//
// }
