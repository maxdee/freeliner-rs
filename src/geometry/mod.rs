pub use self::point::Point;
pub use self::segment::*;

pub mod point;
pub mod segment;

#[derive(Debug)]
pub struct Context {
    pub groups : Vec<SegmentGroup>,
}


impl Context {
    pub fn start(&mut self){
        // add a segment group
        // self.new_group();
        let mut g = SegmentGroup {id: 0, segments: Vec::new(),};

        g.add_seg(Point::new(10.0, 10.0, 0.0), Point::new(100.0, 100.0, 0.0));
        self.groups.push(g);
        // let mut seg = SegmentTypes::straight(StraightSegment{ a: Point::new(10.0, 10.0, 0.0),
        //                                                       b: });
        // group.segments.push(seg);
        // self.groups.push(group);
    }

    // pub fn get_group(&self, i: u32) -> Option<SegmentGroup> {
    //     self.groups.get(i)
    // }

    pub fn new_group(&mut self) {
        let index = self.groups.len() as u32;
        let mut g = SegmentGroup {id: index, segments: Vec::new(),};
        self.groups.push(g);
    }

    // pub fn get_last_group<'a>(self) -> &'a mut Option<SegmentGroup> {
    //     // self.groups.get(self.groups.len()-1)
    //     // None
    // }

    // fn save(&self, path: String) {
    //
    // }
    //
    // fn load(&mut self, path: String) {
    //
    // }

}

#[derive(Debug)]
pub struct SegmentGroup {
    id: u32,
    pub segments: Vec<SegmentType>,
}

impl SegmentGroup {
    pub fn add_seg(&mut self, a: Point, b: Point){
        let mut seg = SegmentType::straight(StraightSegment{ a: a, b: b});
        self.segments.push(seg);
    }
    // pub fn get_segment(i: u32) -> Option
}
