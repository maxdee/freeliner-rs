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

use std::collections::HashMap;
use std::iter::FromIterator;

pub use self::animate::{Animator, RenderItem};
pub use self::geometry::Geometry;
pub use self::input::Input;
pub use self::cmd::*;

pub struct Freeliner {
    pub input: Input,
    pub state: State,
    // pub animator: Animator,
}

impl Freeliner {
    pub fn get_frame(&mut self) -> Vec<RenderItem> {
        self.state
            .context_map
            .iter_mut()
            .flat_map(|ctx| ctx.1.animator.animate(&ctx.1.geometry) )
            .collect()
    }
}

// pub const COMMAND_FACTORY: CommandFactory = CommandFactory::default();

impl Default for Freeliner {
    fn default() -> Self {
        let mut input = Input::new();
        let mut state = State::new();
        let mut context = Context::new("default".to_string());
        state.add_context(context);

        Freeliner { input, state }
    }

}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub context_map: HashMap<String, Context>,
    // add context id usizes?
}

impl State {
    pub fn new() -> Self {
        Self {
            context_map: HashMap::new(),
        }
    }
    pub fn add_context(&mut self, context: Context) {
        self.context_map
            .insert(context.get_name().to_string(), context);
    }
    pub fn get_context(&mut self, name: &str) -> Option<&mut Context> {
        self.context_map.get_mut(name)
    }
    // pub fn borrow_context(&mut self, key : String) -> Result
}

/// A context is an
#[derive(Serialize, Deserialize)]
pub struct Context {
    pub name: String,
    pub geometry: Geometry,
    pub animator: Animator,
}

impl Context {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn new(name: String) -> Self {
        Self {
            name,
            geometry: Geometry::new(),
            animator: Animator::default().populate(),
        }
    }
}
