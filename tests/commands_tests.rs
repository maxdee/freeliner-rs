extern crate freeliner;

mod common;

#[test]
pub fn new_group_cmd() {
    let mut freeliner = common::setup_freeliner();
    freeliner.input.string_command(&mut freeliner.state, "default newgroup".to_string());
    assert_eq!(1, freeliner.state.geom.groups.len());
}


#[test]
pub fn add_point_cmd() {
    let mut freeliner = common::setup_freeliner();
    freeliner.input.string_command(&mut freeliner.state, "default newgroup".to_string());
    freeliner.input.string_command(&mut freeliner.state, "default addpoint 0 20 33.4".to_string());
    freeliner.input.string_command(&mut freeliner.state, "default addpoint 0 20 33.4 -42.0".to_string());

    assert_eq!(20.0, freeliner.state.geom.points[0].x);
    assert_eq!(33.4, freeliner.state.geom.points[0].y);
    assert_eq!(-42.0, freeliner.state.geom.points[1].z);
}


#[test]
pub fn remove_point_cmd() {
    let mut freeliner = common::setup_freeliner();
    freeliner.input.string_command(&mut freeliner.state, "default newgroup".to_string());
    freeliner.input.string_command(&mut freeliner.state, "default addpoint 0 10 10".to_string());
    freeliner.input.string_command(&mut freeliner.state, "default addpoint 0 12 13 42".to_string());
    // check the points were added
    assert_eq!(1, freeliner.state.geom.groups[0].segments.len());
    freeliner.input.string_command(&mut freeliner.state, "default removepoint 0".to_string());
    assert_eq!(0, freeliner.state.geom.groups[0].segments.len());
}




// #[test]
// pub fn save_state_cmd() {
//
// }
