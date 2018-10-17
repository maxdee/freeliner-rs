#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn get_version() -> &'static str {
    "0.0000001"
}

// geometry submodule
pub mod animate;
pub mod cmd;
pub mod geometry;
pub mod input;
// instance submodule,
// pub mod instance;

pub use self::geometry::Data;
pub use self::input::Input;
pub use self::animate::Animator;

pub struct Freeliner {
    pub input: Input,
    pub state: State,
    pub animator: Animator,
}

impl Freeliner {
    pub fn new() -> Self {
        Freeliner {
            input: Input::new(),
            state: State::new(),
            animator: Animator::new(),
        }
    }
}

pub struct State {
    pub geom: geometry::Data,
}

impl State {
    fn new() -> Self {
        State {
            geom: geometry::Data::new(),
        }
    }
}
