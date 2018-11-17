extern crate serde;
extern crate serde_json;

pub use super::State;
pub use geometry::*;

use self::serde_json::Error;

use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
// in this command pattern, I would need to have indexes or keys to args

pub struct CommandConsumer {
    pub log: Vec<Box<Command>>,
    // fn parse_from_osc?
    // implement command recording/playing
}

impl Default for CommandConsumer {
    fn default() -> Self {
        Self { log: Vec::new() }
    }
}
// Command consumer validates and executes commands
impl CommandConsumer {
    pub fn validate_and_exec(&mut self, state: &mut State, cmd: Result<Box<Command>, CmdError>) {
        match cmd {
            Ok(c) => self.exec(state, c),
            Err(e) => println!("invalid cmd: {}", e),
        }
    }

    pub fn exec(&mut self, state: &mut State, mut cmd: Box<Command>) {
        cmd.execute(state).unwrap_or_else(|err| {
            // eprintln!("CMD Fail : {}", err)
            println!("CMD Fail : {}", err)
        });
        println!("{}", cmd.to_string());
        self.log.push(cmd);
    }

    pub fn get_log(&self) -> Vec<String> {
        self.log.iter().map(|cmd| cmd.to_string()).collect()
    }
}

pub struct CommandFactory {
    pub command_list: Vec<Box<Command>>,
}

impl Default for CommandFactory {
    fn default() -> Self {
        let list = Vec::new();
        Self { command_list: list }
    }
}

impl CommandFactory {
    pub fn populate(mut self) -> Self {
        // self.command_list.push(Box::new(SaveStateCmd::default()));
        // self.command_list.push(Box::new(LoadStateCmd::default()));
        // self.command_list.push(Box::new(AddPointCmd::default()));
        // self.command_list.push(Box::new(RemovePointCmd::default()));
        // self.command_list.push(Box::new(BreakLineCmd::default()));
        // self.command_list.push(Box::new(NewGroupCmd::default()));
        // self.command_list.push(Box::new(NudgePointCmd::default()));
        // self
    }
    pub fn string_to_command(&self, string: String) -> Result<Box<Command>, CmdError> {

        let potential_commands = self
            .command_list
            .iter()
            .for_each(|cmd| cmd.from_string(&string))
            .filter(|res| res.err() != Some(CmdError::NoMatch))
            .collect();

        potential_commands.iter().for_each(|thing| println!("{:?}", thing));
        // if let Some(first) = string.split_whitespace().next() {
        //     match first {
        //         "savestate" => Ok(T::from_string(string)),
        //         // "loadstate" => Ok(LoadStateCmd::from_string(string)),
        //         // "addpoint" => Ok(AddPointCmd::from_string(string)),
        //         // "removepoint" => Ok(RemovePointCmd::from_string(string)),
        //         // "newgroup" => Ok(NewGroup::from_string(string)),
        //         // "nudgepoint" => Ok(NudePointCmd::from_string(string)),
        //         _ => Err(format!("unknown command : {}", string)),
        //     }
        // } else {
        Err("unknown command".to_string())
        // }
    }

    fn primary_match(&self, string: String) -> Result<Box<Command>, CmdError> {
        match string.split_whitespace().next() {
            SaveStateCmd::get_keyword() => Ok(Box::new(SaveStateCmd::from_string(string))),
            LoadStateCmd::get_keyword() => Ok(Box::new(LoadStateCmd::from_string(string))),
            AddPointCmd::get_keyword() =>Ok(Box::new( AddPointCmd::from_string(string))),
            RemovePointCmd::get_keyword() => Ok(Box::new(RemovePointCmd::from_string(string))),
            BreakLineCmd::get_keyword() => Ok(Box::new(BreakLineCmd::from_string(string))),
            NewGroupCmd::get_keyword() =>Ok(Box::new( NewGroupCmd::from_string(string))),
            NudgePointCmd::get_keyword() => Ok(Box::new(NudgePointCmd::from_string(string))),
            _ => Err(CmdError::NoMatch),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
pub trait Command {
    fn from_string(args: String) -> Result<Box<Self>, CmdError>;
    fn get_keyword() -> &'static str;
    fn execute(&mut self, state: &mut State) -> Result<(), &str>;
    fn to_string(&self) -> String;
    // where
    //     Self: Sized;
    // to_json??
}

#[derive(Debug)]
pub enum CmdError {
    NoMatch,
    Malformed,
    NoCommand(&'static str),
    NotImplemented(&'static str),
    FileError(),
}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CmdError::NoMatch => f.write_str("not a match"),
            CmdError::NoCommand(ref string) => f.write_str(string),
            CmdError::NotImplemented(ref string) => f.write_str(string),
            _ => f.write_str("unknown error :("),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct SaveStateCmd {
    pub filepath: String,
    // keyword: &'static str,
}

impl SaveStateCmd {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }
}

impl Command for SaveStateCmd {
    fn get_keyword() -> &'static str {
        "savestate"
    }
    // static KEYWORD: &'static str = "savestate";
    // const KEYWORD: &str = "savestate";
    // if supplied a filename or use default
    fn from_string(args: String) -> Result<Box<Self>, CmdError> {
        if args.contains(Self::get_keyword()) {
            match args.split_whitespace().nth(1) {
                Some(filepath) => Ok(Box::new(Self::new(filepath.to_string()))),
                _ => Ok(Self::new("default.json".to_string())),
            }
        } else {
            Err(CmdError::NoMatch)
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
        format!("savestate {}", self.filepath,)
    }
}

// not sure about the benefits...
// impl fmt::Display for SaveStateCmd {
//     fn fmt(&self, f: &mut fmt::Formatter) ->
//         fmt::Result {
//             write!(f, "{}", self.to_string())
//     }
// }

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct LoadStateCmd {
    pub filepath: String,
}

impl LoadStateCmd {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }
}

impl Command for LoadStateCmd {
    fn get_keyword() -> &'static str {
        "loadstate"
    }
    // if supplied a filename or use default
    fn from_string(args: String) -> Result<Box<Self>, CmdError> {
        if args.contains(Self::get_keyword()) {
            let mut split = args.split(" ");
            match split.nth(1) {
                Some(filepath) => Ok(Self::new(filepath.to_string())),
                _ => Ok(Self::new("default.json".to_string())),
            }
        } else {
            Err(CmdError::NoMatch)
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
        format!("savestate -f={}", self.filepath,)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct AddPointCmd {
    // pub group: &mut SegmentGroup,
    pub index: Option<usize>,
    pub group: usize,
    pub new_point: Point,
}

impl AddPointCmd {
    pub fn new(group: usize, new_point: Point) -> Self {
        Self {
            index: None,
            group,
            new_point,
        }
    }
}

impl Command for AddPointCmd {
    fn get_keyword() -> &'static str {
        "addpoint"
    }
    // addpoint -g=3 -xy=20,30
    fn from_string(args: String) -> Result<Box<Self>, CmdError> {
        if args.contains(Self::get_keyword()) {
            Ok(Self::new(0, Point::default()))
        // can just do let x: f32 = arg.parse(); and check for error
        } else {
            Err(CmdError::NoMatch)
        }
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
#[derive(Debug, Default)]
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
    fn get_keyword() -> &'static str {
        "removepoint"
    }
    fn from_string(args: String) -> Result<Box<Self>, CmdError> {
        if args.contains(Self::get_keyword()) {
            Ok(Self::new(0))
        } else {
            Err(CmdError::NoMatch)
        }
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
#[derive(Debug, Default)]
pub struct BreakLineCmd {
    // pub group: &mut SegmentGroup,
    pub group: usize,
    pub new_point: Point,
}

impl BreakLineCmd {
    pub fn new(group: usize, new_point: Point) -> Self {
        Self { group, new_point }
    }
}

impl Command for BreakLineCmd {
    fn get_keyword() -> &'static str {
        "breakline"
    }
    fn from_string(args: String) -> Result<Box<Self>, CmdError> {
        if args.contains(Self::get_keyword()) {
            Ok(Self::new(0, Point::default()))
        } else {
            Err(CmdError::NoMatch)
        }
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
#[derive(Debug, Default)]
pub struct NewGroupCmd {}

impl NewGroupCmd {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for NewGroupCmd {
    fn get_keyword() -> &'static str {
        "newgroup"
    }
    fn from_string(args: String) -> Result<Box<Self>, CmdError> {
        if args.contains(Self::get_keyword()) {
            Ok(Self::new())
        } else {
            Err(CmdError::NoMatch)
        }
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
#[derive(Debug, Default)]
pub struct NudgePointCmd {
    // pub group: &mut SegmentGroup,
    pub point: usize,
    pub nudge: Point,
}

impl NudgePointCmd {
    pub fn new(point: usize, nudge: Point) -> Self {
        Self { point, nudge }
    }
}

impl Command for NudgePointCmd {
    fn get_keyword() -> &'static str {
        "nudgepoint"
    }
    fn from_string(args: String) -> Result<Box<Self>, CmdError> {
        if args.contains(Self::get_keyword()) {
            Ok(Self::new(0, Point::default()))
        } else {
            Err(CmdError::NoMatch)
        }
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
