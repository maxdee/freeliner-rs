pub use self::point::Point;
pub use self::segment::*;

pub mod point;
pub mod segment;

#[derive(Default)]
pub struct Data {
    pub groups: Vec<SegmentGroup>,
}

impl Data {
    // make new data
    pub fn new() -> Self {
        let mut d = Data{groups : Vec::new()};
        d.groups.push(SegmentGroup::new(0));
        d
    }
    // access data
    pub fn get_group(&mut self, index: usize) -> Option<&mut SegmentGroup> {
        if self.has_index(index) {
            Some(&mut self.groups[index])
        } else {
            None
        }
    }

    pub fn has_index(&self, index: usize) -> bool {
        index < self.groups.len()
    }
}

////////////////////////////////////////////////////////////
// #[derive(Debug)]
// #[derive(Default)]
// pub struct Handler {
//     pub cursor_position: Point,
//     pub previous_point: Point,
//     pub selected_group_index: usize,
// }
//
// impl Default for Handler {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Handler {
//     pub fn new() -> Handler {
//         Handler {
//             cursor_position: Point::new_2d(0.0, 0.0),
//             previous_point: Point::new_2d(0.0, 0.0),
//             selected_group_index: 0,
//         }
//     }
//
//     pub fn mouse_moved(&mut self, x: f32, y: f32) {
//         let p = Point::new_2d(x, y);
//         self.cursor_position.set(&p);
//     }
//
//     pub fn left_click(&mut self, data: &mut Data, x: f32, y: f32) {
//         if !data.has_index(self.selected_group_index) {
//             // let mut sg = ;
//             data.groups.push(SegmentGroup::new(self.selected_group_index));
//         }
//         // try to access selected group
//         if let Some(sg) = data.get_group(self.selected_group_index) {
//             let click = Point::new_2d(x, y);
//             let mut seg = StraightSegment::new(&self.previous_point, &click);
//             // println!("new seg {:?}", seg);
//             sg.segments.push(seg);
//             self.previous_point.set(&click);
//         }
//         // selected_group.left_click(x,y);
//         // println!("made a new group {}", selected_group.index);
//     }
// }
