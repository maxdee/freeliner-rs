use super::point::*;

#[derive(Debug)]
pub struct SegmentGroup {
    pub index: usize,
    pub segments: Vec<StraightSegment>,
    previous_point: Option<Point>,
    is_selected: bool,
}

impl SegmentGroup {
    pub fn new(index: usize) -> SegmentGroup {
        SegmentGroup {
            index,
            segments: Vec::new(),
            previous_point: None,
            is_selected: false,
        }
    }
    // only accessors??
}

pub trait SegStuff {
    // or get_pos trait???
    fn get_pos(&self, u: f32) -> Point;
    fn length(&self) -> f32;
}

// #[derive(Debug)]
// pub enum SegmentType {
//     straight(StraightSegment),
//     bezier(BezierSegment),
//     nested,
// }

// impl SegmentType {
//     fn get_pos(&self, u: f32) -> Point{}
// }

#[derive(Debug)]
pub struct StraightSegment {
    pub a: Point,
    pub b: Point,
}

impl StraightSegment {
    pub fn new(aa: &Point, bb: &Point) -> Self {
        StraightSegment {
            a: Point::copy(aa),
            b: Point::copy(bb),
        }
    }
}

impl SegStuff for StraightSegment {
    fn get_pos(&self, unit: f32) -> Point {
        Point::lerp(&self.a, &self.b, unit)
    }
    fn length(&self) -> f32 {
        self.a.dist(&self.b)
    }
}

pub fn interpolator<T: SegStuff>(seg: &T, u: f32) -> Point {
    // do different stuff for different type of segments??
    // match types?
    seg.get_pos(u)
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
    // a and ah could be a segment? as we will interpolate on it...
    a: Point,
    b: Point,
    ah: Point,
    bh: Point,
}

// #[derive(Debug)]
pub struct NestedSegment {
    // segments: Vec<SegmentType>,
}

// pub fn scale(seg_type: SegmentTypes){
//
// }
