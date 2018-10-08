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

pub use self::geometry::{Data, Handler};
pub use self::timer::Timer;
pub use self::input::Input;

pub struct Freeliner {
    timer : Timer,
    geometric_data: geometry::Data,
    geometric_handler: geometry::Handler,

}
