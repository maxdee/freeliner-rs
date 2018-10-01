#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn get_version() -> &'static str{
    "0.0000001"
}

// geometry submodule
pub mod geometry;
// instance submodule,
// pub mod instance;
