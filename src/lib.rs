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

pub use self::animate::Animator;
pub use self::geometry::Geometry;
pub use self::input::Input;

pub struct Freeliner {
    pub input: Input,
    pub state: State,
    // pub animator: Animator,
}
pub use self::cmd::*;

pub const COMMAND_FACTORY: CommandFactory = CommandFactory::default();

impl Default for Freeliner {
    fn default() -> Self {
        let mut input = Input::new();
        let mut state = State::new();
        let mut context = Context::new("default".to_string());
        state.add_context(context);

        Freeliner {
            input,
            state,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    context_map: HashMap<String, Context>,
    // add context id usizes?
}

impl State {
    pub fn new() -> Self {
        Self {
            context_map : HashMap::new(),
        }
    }
    pub fn add_context(&mut self, context : Context) {
        self.context_map.insert(context.get_name().to_string(), context);
    }
    pub fn get_contexts(&mut self, name : &str) -> Vec<Option<&mut Context>> {
        let mut ctxs = Vec::new();
        ctxs.push(self.context_map.get_mut(name));
        ctxs
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
            animator: Animator::default(),
        }
    }
}
