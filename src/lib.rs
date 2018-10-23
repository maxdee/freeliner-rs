#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


#[macro_use]
extern crate serde_derive;

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

pub use self::animate::Animator;
pub use self::geometry::Data;
pub use self::input::Input;

pub struct Freeliner {
    pub input: Input,
    pub state: State,
    pub animator: Animator,
}


impl Default for Freeliner {
    fn default() -> Self {
        Freeliner {
            input: Input::new(),
            state: State::new(),
            animator: Animator::default(),
        }
    }
}
#[derive(Serialize, Deserialize)]
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
