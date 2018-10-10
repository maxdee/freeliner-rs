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
pub mod geometry;
pub mod timer;
pub mod input;
pub mod cmd;
// instance submodule,
// pub mod instance;

pub use self::geometry::{Data};
pub use self::timer::Timer;
pub use self::input::Input;

pub struct Freeliner {
    pub input: Input,
    pub state: State,
}
impl Freeliner {
    pub fn new() -> Self{
        Freeliner{input: Input::new(), state: State::new()}
    }
}

pub struct State {
    timer : Timer,
    pub geometric_data: geometry::Data,
}

impl State {
    fn new() -> Self {
        State {timer: Timer::new(), geometric_data: geometry::Data::new()}
    }
}
