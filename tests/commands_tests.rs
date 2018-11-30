extern crate freeliner;

mod common;

#[test]
pub fn new_group_cmd() {
    let mut freeliner = common::setup_freeliner();
    freeliner.input.string_command(&mut freeliner.state, "default newgroup".to_string());
    let context = freeliner.state.get_context("default").unwrap();
    assert_eq!(1, context.geometry.groups.len());
}


#[test]
pub fn add_point_cmd() {
    let mut freeliner = common::setup_freeliner();
    freeliner.input.string_command(&mut freeliner.state, "default newgroup".to_string());
    freeliner.input.string_command(&mut freeliner.state, "default addpoint 0 20 33.4".to_string());
    freeliner.input.string_command(&mut freeliner.state, "default addpoint 0 20 33.4 -42.0".to_string());
    let context = freeliner.state.get_context("default").unwrap();
    assert_eq!(20.0, context.geometry.points[0].x);
    assert_eq!(33.4, context.geometry.points[0].y);
    assert_eq!(-42.0, context.geometry.points[1].z);
}


#[test]
pub fn remove_point_cmd() {
    let mut freeliner = common::setup_freeliner();
    freeliner.input.string_command(&mut freeliner.state, "default newgroup".to_string());
    freeliner.input.string_command(&mut freeliner.state, "default addpoint 0 10 10".to_string());
    freeliner.input.string_command(&mut freeliner.state, "default addpoint 0 12 13 42".to_string());
    // check the points were added
    {
        let context = freeliner.state.get_context("default").unwrap();
        assert_eq!(1, context.geometry.groups[0].segments.len());
    }

    freeliner.input.string_command(&mut freeliner.state, "default removepoint 0".to_string());
    let context = freeliner.state.get_context("default").unwrap();
    assert_eq!(0, context.geometry.groups[0].segments.len());
}

#[test]
pub fn save_load_cmd() {
    let mut freeliner = common::setup_freeliner();
    freeliner.input.string_command(&mut freeliner.state, "default newgroup".to_string());
    freeliner.input.string_command(&mut freeliner.state, "default addpoint 0 10 11".to_string());
    freeliner.input.string_command(&mut freeliner.state, "default addpoint 0 20 21".to_string());
    freeliner.input.string_command(&mut freeliner.state, "* savestate _test_state.json".to_string());

    let mut freeliner = common::setup_freeliner();
    freeliner.input.string_command(&mut freeliner.state, "* loadstate _test_state.json".to_string());
    let context = freeliner.state.get_context("default").unwrap();

    assert_eq!(10.0, context.geometry.points[0].x);
}

#[test]
pub fn node_tree_graph_cmd() {
    let mut freeliner = common::setup_freeliner();
    freeliner.input.string_command(&mut freeliner.state,
        "default tree A graph iter-1010 segs-1013 enter1012 brush-1011".to_string());
    let context = freeliner.state.get_context("default").unwrap();
    assert_eq!(1013, context.animator.node_trees.get("A").unwrap().nodes.get(1).unwrap().get_id());
}
