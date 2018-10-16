pub use self::point::Point;
pub use self::segment::*;

pub mod point;
pub mod segment;

pub const NONE_GROUP: usize = 0;

#[derive(Default, Debug)]
pub struct Data {
    pub points: Vec<Point>,
    pub segs: Vec<Segment>,
    pub groups: Vec<Group>,
}

impl Data {
    // make new data
    pub fn new() -> Self {
        Self{
            points: Vec::new(),
            segs: Vec::new(),
            groups: Vec::new(),
        }
    }
}

// flat flat
#[derive(Debug)]
pub struct Segment {
    pub point_a: usize,
    pub point_b: usize,
}

impl Segment {
    pub fn new(point_a: usize, point_b: usize) -> Self {
        Self{point_a, point_b}
    }
}

#[derive(Debug)]
pub struct Group {
    pub segments: Vec<usize>,
    pub index: usize,
    pub previous_point: Option<usize>,
}

impl Group {
    pub fn new(index: usize) -> Self{
        Self{segments: Vec::new(), index, previous_point: None}
    }
}
