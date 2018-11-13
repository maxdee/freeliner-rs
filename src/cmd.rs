extern crate serde;
extern crate serde_json;

pub use super::State;
pub use geometry::*;

use self::serde_json::Error;

use std::fs::File;
use std::fmt;
use std::error;
use std::io::prelude::*;
// in this command pattern, I would need to have indexes or keys to args

pub struct CommandConsumer {
    pub log: Vec<Box<Command>>,
    // fn parse_from_osc?
    // implement command recording/playing
}

impl Default for CommandConsumer {
    fn default() -> Self {
        Self {
            log: Vec::new(),
        }
    }
}
// Command consumer validates and executes commands
impl CommandConsumer {
    pub fn validate_and_exec<T, E> (&mut self, state: &mut State, cmd: Result<T, E>)
        where T: Command + 'static,
              E: std::fmt::Display,
    {
        match cmd {
            Ok(c) => self.exec(state, c),
            Err(e) => println!("invalid cmd: {}", e),
        }
    }

    pub fn exec<T: 'static>(&mut self, state: &mut State, mut cmd: T)
    where
        T: Command,
    {
        cmd.execute(state).unwrap_or_else(|err| {
            // eprintln!("CMD Fail : {}", err)
            println!("CMD Fail : {}", err)
        });
        println!("{}", cmd.to_string());
        self.log.push(Box::new(cmd));
    }

    pub fn get_log(&self) -> Vec<String> {
        self.log.iter().map(|cmd| cmd.to_string()).collect()
    }

}

pub struct CommandFactory;

impl CommandFactory {
    pub fn string_to_command<T> (string: String) -> Result<T, String>
        where T: Command,
    {
        // let first = string.split_whitespace().next();
        // match first {
        //     Some(f) => match f {
        //         "savestate" => Ok({let mut cmd : Command = SaveStateCmd::from_string(string);cmd}),
        //         "loadstate" => Ok(LoadStateCmd::from_string(string)),
        //         "addpoint" => Ok(AddPointCmd::from_string(string)),
        //         "removepoint" => Ok(RemovePointCmd::from_string(string)),
        //         "newgroup" => Ok(NewGroup::from_string(string)),
        //         "nudgepoint" => Ok(NudePointCmd::from_string(string)),
        //
        //         _ => Err(format!("unknown command : {}", string)),
        //     },
        //     None => println!(" {}", string),
        // }
        Err("unknown command".to_string())
    }
}

/////////////////////////////////////////////////////////////////////////////////////////

pub trait Command {
    fn from_string(args: String) -> Self where Self: Sized;
    fn execute(&mut self, state: &mut State) -> Result<(), &str>;
    fn to_string(&self) -> String;
    // to_json??
}

// #[derive(Debug)]
// pub struct FileNotFound {
//     file: String,
// }
//
// impl fmt::Display for FileNotFound {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "no such file {}", self.file)
//     }
// }
//
// impl error::Error for FileNotFound {
//     fn description(&self) -> &str {
//         "no such file"
//     }
//
//     fn cause(&self) -> Option<&error::Error> {
//         None
//     }
// }
/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct SaveStateCmd {
	pub filepath: String,
}

impl SaveStateCmd {
	pub fn new(filepath: String) -> Self{
		Self{ filepath }
	}
}

impl Command for SaveStateCmd {

    // if supplied a filename or use default
    fn from_string(args: String) -> Self {
        let mut split = args.split(" ");
        match split.nth(1) {
            Some(filepath) => Self::new(filepath.to_string()),
            _ => Self::new("default.json".to_string()),
        }
    }

	fn execute(&mut self, state: &mut State) -> Result<(), &str> {
		let j = serde_json::to_vec(state).unwrap();
		let f = self.filepath.clone();
		let mut file = File::create(f).unwrap();
		file.write_all(&j).unwrap();
        Ok(())
    }

    fn to_string(&self) -> String {
        format!(
            "savestate -f={}",
            self.filepath,
        )
    }
}

// not sure about the benefits...
impl fmt::Display for SaveStateCmd {
    fn fmt(&self, f: &mut fmt::Formatter) ->
        fmt::Result {
            write!(f, "savestate -f={}", self.filepath)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct LoadStateCmd {
	pub filepath: String,
}

impl LoadStateCmd {
	pub fn new(filepath: String) -> Self{
		Self{ filepath }
	}
}

impl Command for LoadStateCmd {
    // if supplied a filename or use default
    fn from_string(args: String) -> Self {
        let mut split = args.split(" ");
        match split.nth(1) {
            Some(filepath) => Self::new(filepath.to_string()),
            _ => Self::new("default.json".to_string()),
        }
    }

	fn execute(&mut self, state: &mut State) -> Result<(), &str> {
		let f = self.filepath.clone();
		let mut file = File::open(f).unwrap();
		let mut contents = String::new();
		file.read_to_string(&mut contents);
		*state = serde_json::from_str(&contents).unwrap();
        Ok(())
    }

    fn to_string(&self) -> String {
        format!(
            "savestate -f={}",
            self.filepath,
        )
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct AddPointCmd {
    // pub group: &mut SegmentGroup,
    pub index: Option<usize>,
    pub group: usize,
    pub new_point: Point,
}

impl AddPointCmd {
    pub fn new(group: usize, new_point: Point) -> Self {
        Self { index: None, group, new_point }
    }
}

impl Command for AddPointCmd {
    // addpoint -g=3 -xy=20,30
    fn from_string(args: String) -> Self {
        Self::new(0, Point::default())
    }
    // const NAME: &str = "addpoint";
    fn execute(&mut self, state: &mut State) -> Result<(), &str> {
        // let group = get_group(state, self.index)?;
        // do this better
        if self.group >= state.geom.groups.len() {
            return Err("no such group"); //format!("no such group {}", self.to_string()));
        }

        let group = &mut state.geom.groups[self.group];
        state.geom.points.push(Point::copy(&self.new_point));

        let new_index = state.geom.points.len() - 1;
        self.index = Some(new_index);

        if group.previous_point.is_some() {
            // push the new point, but if is snapped, then dont...
            // add a new segment
            state
                .geom
                .segs
                .push(Segment::new(group.previous_point.unwrap(), new_index));
            // add the segment to the group
            group.segments.push(state.geom.segs.len());
        // group.previous_point = Some(new_index);
        } else {
        }
        group.previous_point = Some(new_index);
        Ok(())
    }
    // fn get_name(&self) -> &'static str {
    // 	"addpoint"
    // }
    fn to_string(&self) -> String {
        format!(
            "addpoint -g={} -xy={},{}",
            self.group, self.new_point.x, self.new_point.y
        )
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct RemovePointCmd {
    // pub group: &mut SegmentGroup,
    pub group: usize,
}

impl RemovePointCmd {
    pub fn new(group: usize) -> Self {
        Self { group }
    }
}

impl Command for RemovePointCmd {
    fn from_string(args: String) -> Self {
        Self::new(0)
    }
    fn execute(&mut self, state: &mut State) -> Result<(), &str> {
        let group = state
            .geom
            .groups
            .get_mut(self.group)
            .ok_or("no such group")?;

        if group.segments.len() > 0 {
            group.segments.pop();
            if group.segments.len() == 0 {
                group.previous_point = None;
            } else {
                group.previous_point =
                    Some(state.geom.segs[group.segments[group.segments.len() - 1]].point_a);
            }
        } else {
            return Err("no points to remove");
        }
        Ok(())
    }

    // fn get_name(&self) -> &'static str {
    // 	"removepoint"
    // }
    fn to_string(&self) -> String {
        format!("removepoint -g={}", self.group)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct BreakLine {
    // pub group: &mut SegmentGroup,
    pub group: usize,
    pub new_point: Point,
}

impl BreakLine {
    pub fn new(group: usize, new_point: Point) -> Self {
        Self { group, new_point }
    }
}

impl Command for BreakLine {
    fn from_string(args: String) -> Self {
        Self::new(0, Point::default())
    }
    fn execute(&mut self, state: &mut State) -> Result<(), &str> {
        let group = &mut state.geom.groups[self.group];
        state.geom.points.push(Point::copy(&self.new_point));
        let new_index = state.geom.points.len() - 1;
        group.previous_point = Some(new_index);
        Ok(())
    }
    // fn get_name(&self) -> &'static str {
    // 	"breakline"
    // }
    fn to_string(&self) -> String {
        format!(
            "breakline -g={} -xy={},{}",
            self.group, self.new_point.x, self.new_point.y
        )
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct NewGroup {}

impl NewGroup {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for NewGroup {
    fn from_string(args: String) -> Self {
        Self::new()
    }
    fn execute(&mut self, state: &mut State) -> Result<(), &str> {
        let i = state.geom.groups.len();
        state.geom.groups.push(Group::new(i));
        Ok(())
    }
    // fn get_name(&self) -> &'static str {
    // 	"newgroup"
    // }
    fn to_string(&self) -> String {
        String::from("newgroup")
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct NudgePoint {
    // pub group: &mut SegmentGroup,
    pub point: usize,
    pub nudge: Point,
}

impl NudgePoint {
    pub fn new(point: usize, nudge: Point) -> Self {
        Self { point, nudge }
    }
}

impl Command for NudgePoint {
    fn from_string(args: String) -> Self {
        Self::new(0, Point::default())
    }
    fn execute(&mut self, state: &mut State) -> Result<(), &str> {
        // let mut point = &mut
        let i = self.point;
        state.geom.points[i] += &self.nudge;
        // point += self.nudge;
        Ok(())
    }
    // fn get_name(&self) -> &'static str {
    // 	"NudgePoint"
    // }
    fn to_string(&self) -> String {
        format!(
            "nudge -p={} -xy={},{}",
            self.point, self.nudge.x, self.nudge.y
        )
    }
}
//
// impl Command for AddPoint {
// 	fn execute(&mut self, state: State) -> Result<(), &str> {
// 		let &mut group = &mut state.geometric_data.groups[self.index];
// 		// self.do(&self.parat)
// 		if let Some(previous_point) = group.previous_point {
// 			let mut seg = StraightSegment::new(&previous_point, &self.new_point);
// 			group.segments.push(seg);
// 		}
// 		else {
// 			let p = Point::copy(&self.new_point);
// 			group.previous_point.unwrap().set(&p);// = Some(p);
// 			// self.group.previous_point.as_mut().unwrap().set(&p);// = Some(p);
// 		}
// 		Ok(())
// 	}
// }
