pub use geometry::Point;
pub use cmd::*;
pub use super::State;



#[derive(Default)]
pub struct Input{
    cursor_position: Point,
    selected_group_index: usize,
    cursor_line: (Point, Point),
    // command_logger: Vec<C>
}

pub const LEFT_BUTTON: usize = 1;
pub const RIGHT_BUTTON: usize = 2;
pub const MIDDLE_BUTTON: usize = 3;



impl Input {
    pub fn new() -> Input {
        Input{
            cursor_position: Point::default(),
            selected_group_index : 0,
            cursor_line: (Point::default(), Point::default()),
        }
    }

    // pub fn key_pressed(&mut self, state: &mut State, ){
    //
    // }

    pub fn mouse_pressed(&mut self, state: &mut State, button: usize, pos: Point){
        println!("Pressed {} at {:?}", button, pos);
        match button {
            LEFT_BUTTON => self.handle_left_click(state, pos),
            RIGHT_BUTTON => self.handle_right_click(state, pos),
            MIDDLE_BUTTON => self.handle_middle_click(state, pos),
            _ => (),
        }
    }

    fn handle_left_click(&mut self, state: &mut State, pos: Point) {
        AddPoint::new(self.selected_group_index, pos).execute(state);
    }

    fn handle_right_click(&mut self, state: &mut State, pos: Point) {
        RemovePoint::new(self.selected_group_index, pos).execute(state);
    }

    fn handle_middle_click(&mut self, state: &mut State, pos: Point) {
        NewGroup::new().execute(state);
        self.selected_group_index = state.geometric_data.groups.len()-1;
    }


    pub fn mouse_moved(&mut self, pos: Point) {
        self.cursor_position.set(&pos);
    }

    pub fn prep_for_gui(&mut self) {
        // cursor_line.1 =
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
