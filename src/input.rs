pub use geometry::Point;
pub use cmd::*;
pub use super::State;

#[derive(Default)]
pub struct Input{
    cursor_position: Point,
    selected_group_index: usize,
    cursor_line: (Point, Point),
    command_log: Vec<Box<Command>>,
    snap_distance: u32,
}

// use enum instead
pub const LEFT_BUTTON: usize = 1;
pub const RIGHT_BUTTON: usize = 2;
pub const MIDDLE_BUTTON: usize = 3;

// fn unbox<T>(value: Box<T>) -> T {
//     *value
// }

impl Input {

    pub fn new() -> Input {
        Input{
            cursor_position: Point::default(),
            selected_group_index : 0,
            cursor_line: (Point::default(), Point::default()),
            command_log: Vec::new(),
            snap_distance: 10,
        }
    }

    // pub fn exec_cmd(&mut self, state: &mut State, mut bx: Box<Command>){
    pub fn exec_cmd<T: 'static>(&mut self, state: &mut State, mut cmd: T)
    where T: Command
    {
        cmd.execute(state);
        self.command_log.push(Box::new(cmd));
    }


    /// Input mouse press event into freeliner.
    pub fn mouse_pressed(&mut self, state: &mut State, button: usize, pos: Point){
        println!("Pressed {} at {:?}", button, pos);
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
        self.exec_cmd( state, AddPointCmd::new(index, pos));
    }

    fn handle_right_click(&mut self, state: &mut State, pos: Point) {
        let index = self.selected_group_index;
        self.exec_cmd(state, RemovePoint::new(index, pos));
    }

    fn handle_middle_click(&mut self, state: &mut State, pos: Point) {
        let index = self.selected_group_index;
        self.exec_cmd(state, BreakLine::new(index, pos));
    }

    pub fn mouse_moved(&mut self, state: &State, pos: Point) {
        self.cursor_position.set(&pos);
        self.snapping(state, &pos);
    }

    pub fn snapping(&mut self, state: &State, pos: &Point) {
        let snapped : Vec<usize> = state.geom.points.iter()
            .enumerate()
            .filter(|point| { point.1.dist(&pos) < self.snap_distance as f32})
            .map(|point| {point.0})
            .collect();
        // println!("{:#?}", snapped);
    }

    pub fn key_pressed(&mut self, state: &mut State, key: u32) {
        match key {
            key if key == VirtualKeyCode::N as u32 => {
                self.exec_cmd(state, NewGroup::new());
                self.selected_group_index = state.geom.groups.len()-1;
            },
            key if key == VirtualKeyCode::L as u32 => {
                // println!("{:#?}", self.command_log);
            },
            _ => {},
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
