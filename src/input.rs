pub use geometry::Point;
pub use cmd::*;
pub use super::State;



#[derive(Default)]
pub struct Input{
    cursor_position: Point,
    selected_group_index: usize,
}

impl Input {
    pub fn new() -> Input {
        Input{cursor_position: Point::default(), selected_group_index : 0}
    }

    pub fn mouse_pressed(&mut self, state: &mut State, button: u8, pos: Point){
        println!("Pressed {} at {:?}", button, pos);
        AddPoint::new(self.selected_group_index, pos).execute(state);
    }

    pub fn mouse_moved(&mut self, pos: Point) {
        self.cursor_position.set(&pos);
    }


    // pub fn left_click(&mut self, data: &mut Data, x: f32, y: f32) {
    //     if !data.has_index(self.selected_group_index) {
    //         // let mut sg = ;
    //         data.groups.push(SegmentGroup::new(self.selected_group_index));
    //     }
    //     // try to access selected group
    //     if let Some(sg) = data.get_group(self.selected_group_index) {
    //         let click = Point::new_2d(x, y);
    //         let mut seg = StraightSegment::new(&self.previous_point, &click);
    //         // println!("new seg {:?}", seg);
    //         sg.segments.push(seg);
    //         self.previous_point.set(&click);
    //     }
    //     // selected_group.left_click(x,y);
    //     // println!("made a new group {}", selected_group.index);
    // }

}
