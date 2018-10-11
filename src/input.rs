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

// use enum instead
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
        AddPointCmd::new(self.selected_group_index, pos).execute(state);
    }

    fn handle_right_click(&mut self, state: &mut State, pos: Point) {
        RemovePoint::new(self.selected_group_index, pos).execute(state);
    }

    fn handle_middle_click(&mut self, state: &mut State, pos: Point) {
        BreakLine::new(self.selected_group_index, pos).execute(state);
    }

    pub fn mouse_moved(&mut self, pos: Point) {
        self.cursor_position.set(&pos);
    }

    pub fn key_pressed(&mut self, state: &mut State, key: u32) {
        match key {
            key if key == VirtualKeyCode::N as u32 => {
                NewGroup::new().execute(state);
                self.selected_group_index = state.geometric_data.groups.len()-1;
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
