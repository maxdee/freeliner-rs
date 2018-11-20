extern crate freeliner;

mod common;

#[test]
pub fn new_group_cmd() {
    let mut freeliner = common::setup_freeliner();
    freeliner.input.string_command(&mut freeliner.state, "newgroup".to_string());
    assert_eq!(1, freeliner.state.geom.groups.len());
}


#[test]
pub fn add_point_cmd() {
    let mut freeliner = common::setup_freeliner();
    freeliner.input.string_command(&mut freeliner.state, "newgroup".to_string());
    freeliner.input.string_command(&mut freeliner.state, "addpoint 0 20 33.4".to_string());
    freeliner.input.string_command(&mut freeliner.state, "addpoint 0 20 33.4 -42.0".to_string());

    assert_eq!(20.0, freeliner.state.geom.points[0].x);
    assert_eq!(33.4, freeliner.state.geom.points[0].y);
    assert_eq!(-42.0, freeliner.state.geom.points[1].z);
}
