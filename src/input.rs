pub use super::State;
pub use cmd::*;
pub use geometry::Point;

pub struct Input {
    pub cursor_position: Point,
    pub selected_group_index: usize,
    pub cursor_line: (Point, Point),
    snap_distance: f32,
    snap_list: Vec<(usize, f32)>,
    consumer: CommandConsumer,
    cmd_factory: CommandFactory,
}

// use enum instead
pub const LEFT_BUTTON: usize = 1;
pub const RIGHT_BUTTON: usize = 2;
pub const MIDDLE_BUTTON: usize = 3;

// fn unbox<T>(value: Box<T>) -> T {
//     *value
// }

impl Input {
    pub fn new() -> Self {
        Self {
            cursor_position: Point::default(),
            selected_group_index: 0,
            cursor_line: (Point::default(), Point::default()),
            snap_distance: 20.0,
            snap_list: Vec::new(),
            consumer: CommandConsumer::default(),
            cmd_factory: CommandFactory::default().populate(),
        }
    }

    // receive string and maybe execute them
    // not sure about stati
    pub fn receive_osc_string(&mut self, state: &mut State, string: String){
        self.string_command(state, string);
        // println!("got a osc command! ----- {}",string);
    }
    pub fn string_command(&mut self, state: &mut State, string: String) {
        let mut res = self.cmd_factory.string_to_command(string);
        match res {
            Ok(c) => self.consumer.exec(state, c),
            Err(e) => println!("error parsing command : {:?}", e),
        }
    }

    /// Input mouse press event into freeliner.
    pub fn mouse_pressed(&mut self, state: &mut State, button: usize) {
        //, _pos: Point) {
        // println!("Pressed {} at {:?}", button, pos);
        let pos = Point::copy(&self.cursor_position);
        match button {
            LEFT_BUTTON => self.handle_left_click(state, pos),
            RIGHT_BUTTON => self.handle_right_click(state, pos),
            MIDDLE_BUTTON => self.handle_middle_click(state, pos),
            _ => (),
        }
        // println!("{:#?}", state.geom);
    }

    fn handle_left_click(&mut self, state: &mut State, pos: Point) {
        let index = self.selected_group_index;
        self.consumer
            .exec(state, Box::new(AddPointCmd::new(index, pos)));
        self.update_cursor_line(state);
    }

    fn update_cursor_line(&mut self, state: &State) {
        if state.geom.groups.is_empty() {
            return;
        }
        if let Some(point) = state.geom.groups[self.selected_group_index].previous_point {
            self.cursor_line.0.set(&state.geom.points[point]);
            self.cursor_line.1.set(&self.cursor_position);
        }
    }

    fn handle_right_click(&mut self, state: &mut State, _pos: Point) {
        let index = self.selected_group_index;
        self.consumer
            .exec(state, Box::new(RemovePointCmd::new(index)));
        self.update_cursor_line(state);
    }

    fn handle_middle_click(&mut self, state: &mut State, pos: Point) {
        let index = self.selected_group_index;
        self.consumer
            .exec(state, Box::new(BreakLineCmd::new(index, pos)));
        self.update_cursor_line(state);
    }

    pub fn mouse_moved(&mut self, state: &State, pos: Point) {
        self.snapping(state, &pos);
        if !self.snap_list.is_empty() {
            let i = self.closest_snap();
            self.cursor_position.set(&state.geom.points[i]);
        } else {
            self.cursor_position.set(&pos);
        }
        self.cursor_line.1.set(&self.cursor_position);
    }

    fn closest_snap(&self) -> usize {
        self.snap_list
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }

    pub fn snapping(&mut self, state: &State, pos: &Point) {
        self.snap_list = state
            .geom
            .points
            .iter()
            .enumerate()
            .map(|point| (point.0, point.1.dist(&pos)))
            .filter(|point_dist| point_dist.1 < self.snap_distance as f32)
            .collect();
        // println!("{:#?}", self.snap_list);
    }

    pub fn nudge(&mut self, state: &mut State, mut amount: Point) {
        if !self.snap_list.is_empty() {
            let i = self.closest_snap();
            amount *= &Point::new_2d(10.0, 10.0);
            self.cursor_position += &amount;
            self.consumer
                .exec(state, Box::new(NudgePointCmd::new(i, amount)));
        }
    }

    pub fn key_pressed(&mut self, state: &mut State, key: u32) {
        match key {
            key if key == VirtualKeyCode::N as u32 => {
                // println!("NEW group");
                self.consumer.exec(state, Box::new(NewGroupCmd::new()));
                self.selected_group_index = state.geom.groups.len() - 1;
                self.update_cursor_line(state);
            }
            key if key == VirtualKeyCode::O as u32 => {
                self.consumer
                    .exec(state, Box::new(LoadStateCmd::new("state.json".to_string())));
                // println!("{:#?}", state.geom);
                // self.cmd.get_log().iter().map(|cmd| println!("{}", cmd));
            }
            key if key == VirtualKeyCode::S as u32 => {
                // let mut c = SaveStateCmd::from_string(String::from("savestate -f=state.json"));
                // self.cmd.validate_and_exec(state, c);
                self.consumer.exec(
                    state,
                    Box::new(SaveStateCmd::new(String::from("state.json"))),
                );
            }
            key if key == VirtualKeyCode::Tab as u32 => {
                self.selected_group_index += 1;
                self.selected_group_index %= state.geom.groups.len();
                self.update_cursor_line(state);
            }
            key if key == VirtualKeyCode::Return as u32 => {
                self.string_command(state, "removepoint 2".to_string());
            }
            key if key == VirtualKeyCode::Up as u32 => {
                self.nudge(state, Point::new_2d(0.0, 1.0));
            }
            key if key == VirtualKeyCode::Down as u32 => {
                self.nudge(state, Point::new_2d(0.0, -1.0));
            }
            key if key == VirtualKeyCode::Left as u32 => {
                self.nudge(state, Point::new_2d(-1.0, 0.0));
            }
            key if key == VirtualKeyCode::Right as u32 => {
                self.nudge(state, Point::new_2d(1.0, 0.0));
            }
            _ => {}
        }
    }

    pub fn prep_for_gui(&mut self) {
        // cursor_line.1 =
    }
}

// from glium
#[derive(PartialEq)]
pub enum VirtualKeyCode {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    Snapshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    Compose,
    Caret,
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}
