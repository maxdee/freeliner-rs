//
use std::fmt::Debug;

//
// pub trait Parameter {
//     fn get_name(&self) -> &str;
//     fn set(&mut self, string: &str);
// }
//
// pub enum Param {
//     Float{name: String, value: f32, min: f32, max: f32, default: f32},
//     Radio{name: String, value: usize, len: usize, default: usize},
// }
//
// impl Parameter for Param {
//     fn get_name(&self) -> &str {
//         match self {
//             Param::Float{name, ..} => &name,
//             Param::Radio{name, ..} => &name,
//             _ => "not a param",
//             // _ => self.name,
//         }
//     }
//     fn set(&mut self, string: &str) {
//         match *self {
//             Param::Float{..} => {
//                 self.value = string.parse::<f32>().unwrap();
//             },
//             Param::Radio{value, ..} => {
//                 value = string.parse::<usize>().unwrap();
//             },
//             _ => (),
//             // _ => self.name,
//         }
//         let haha = 2;
//     }
// }

// pub trait Parameter {
//
// }
#[derive(Debug, Serialize, Deserialize)]

pub struct Param<T> {
    pub name: String,
    pub value: T,
    pub default: T,
}

impl<T> Param<T> {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_value(&self) -> &T {
        &self.value
    }
    pub fn parse_string(&mut self, string: &str)
    where
        T: std::str::FromStr + Copy + std::fmt::Display,
    {
        if let Ok(val) = string.parse::<T>() {
            self.value = val;
            println!("param {} set to {}", self.name, self.value);
        } else {
            self.value = self.default;
        }
    }
}
