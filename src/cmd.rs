extern crate serde;
extern crate serde_json;

pub use super::State;
pub use geometry::*;

use self::serde_json::Error;

use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum CmdError {
    NoMatch,
    Malformed(String),
    NoCommand(String),
    NotImplemented(String),
    NoSpawner(String),
    SomeError(String),
    FileError(),
    NoFile,
}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CmdError::NoMatch => f.write_str("not a match"),
            CmdError::NoCommand(ref string) => f.write_str(&format!("no command : {:?}", string)),
            CmdError::NotImplemented(ref string) => f.write_str(string),
            _ => f.write_str("unknown error :("),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////

pub struct CommandConsumer {
    pub log: Vec<Box<Command>>,
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
/////////////////////////////////////////////////////////////////////////////////////////
// refactor the command factory as a command with sub commands??
pub struct CommandFactory {
    cmd_map: HashMap<&'static str, Box<Command>>,
}

impl Default for CommandFactory {
    fn default() -> Self {
        let mut cmd_map: HashMap<&'static str, Box<Command>> = HashMap::new();
        Self { cmd_map }
    }
}

impl CommandFactory {
    pub fn populate(mut self) -> Self {
        self.add_cmd(Box::new(SaveStateCmd::default()));
        self.add_cmd(Box::new(LoadStateCmd::default()));
        self.add_cmd(Box::new(NewGroupCmd::default()));
        self.add_cmd(Box::new(AddPointCmd::default()));
        self.add_cmd(Box::new(RemovePointCmd::default()));
        self.add_cmd(Box::new(NudgePointCmd::default()));
        self
    }

    pub fn add_cmd(&mut self, cmd: Box<Command>) {
        self.cmd_map.insert(cmd.get_keyword(), cmd);
    }

    pub fn string_to_command(&self, string: String) -> Result<Box<Command>, CmdError> {
        if let Some(keyword) = string.split_whitespace().nth(0) {
            if let Some(cmd) = self.cmd_map.get(keyword){
                return cmd.parse_string(&string);
            }
        }
        Err(CmdError::NoCommand(string))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////

pub trait Command: fmt::Debug {
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError>;
    fn execute(&mut self, state: &mut State) -> Result<(), &str>;
    fn to_string(&self) -> String;
    fn get_keyword(&self) -> &'static str;
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct SaveStateCmd {
    pub filepath: String,
}

impl SaveStateCmd {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }
}

impl Command for SaveStateCmd {
    fn get_keyword(&self) -> &'static str {
        "savestate"
    }
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        match args.split_whitespace().nth(1) {
            Some(filepath) => {
                Ok(Box::new(Self::new(filepath.to_string())))
            },
            _ => Err(CmdError::NoFile),
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
    fn get_keyword(&self) -> &'static str {
        "loadstate"
    }
    // if supplied a filename or use default
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        match args.split_whitespace().nth(1) {
            Some(filepath) => {
                Ok(Box::new(Self::new(filepath.to_string())))
            },
            _ => Err(CmdError::NoFile),
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
        format!("loadstate -f={}", self.filepath,)
    }
}


// All commands related to geometry state

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
    fn get_keyword(&self) -> &'static str {
        "addpoint"
    }
    // addpoint -g=3 -xy=20,30
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        let mut split = args.split_whitespace();
        split.next();
        let group = split.next()
            .ok_or(CmdError::Malformed(format!("missing group arg : {}", args)))?
            .parse::<usize>()
            .ok()
            .ok_or(CmdError::Malformed(format!("group arg not usize : {}", args)))?;

        let xpos = split.next()
            .ok_or(CmdError::Malformed(format!("missing xpos : {}", args)))?
            .parse::<f32>()
            .ok()
            .ok_or(CmdError::Malformed(format!("xpos not parseable : {}", args)))?;

        let ypos = split.next()
            .ok_or(CmdError::Malformed(format!("missing ypos : {}", args)))?
            .parse::<f32>()
            .ok()
            .ok_or(CmdError::Malformed(format!("ypos not parseable : {}", args)))?;

        let zpos = split.next()
            .unwrap_or("0.0")
            .parse::<f32>()
            .unwrap_or(0.0);

        Ok(Box::new(Self::new(group, Point::new(xpos, ypos, zpos))))
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
    fn to_string(&self) -> String {
        format!(
            "addpoint -g={} -xy={},{},{}",
            self.group, self.new_point.x, self.new_point.y, self.new_point.z
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
    fn get_keyword(&self) -> &'static str {
        "removepoint"
    }

    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        if let Some(group) = args.split_whitespace().nth(1) {
            if let Ok(index) = group.parse::<usize>() {
                Ok(Box::new(Self::new(index)))
            } else {
                Err(CmdError::Malformed(args.to_string()))
            }
        } else {
            Err(CmdError::Malformed(args.to_string()))
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

    fn to_string(&self) -> String {
        format!("removepoint {}", self.group)
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
    fn get_keyword(&self) -> &'static str {
        "breakline"
    }
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        Err(CmdError::NotImplemented(args.to_string()))
    }
    fn execute(&mut self, state: &mut State) -> Result<(), &str> {
        let group = &mut state.geom.groups[self.group];
        state.geom.points.push(Point::copy(&self.new_point));
        let new_index = state.geom.points.len() - 1;
        group.previous_point = Some(new_index);
        Ok(())
    }

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
    fn get_keyword(&self) -> &'static str {
        "newgroup"
    }
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        Ok(Box::new(Self::new()))
    }
    fn execute(&mut self, state: &mut State) -> Result<(), &str> {
        let i = state.geom.groups.len();
        state.geom.groups.push(Group::new(i));
        Ok(())
    }
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
    fn get_keyword(&self) -> &'static str {
        "nudgepoint"
    }
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        Err(CmdError::NotImplemented(args.to_string()))
    }
    fn execute(&mut self, state: &mut State) -> Result<(), &str> {
        // let mut point = &mut
        let i = self.point;
        state.geom.points[i] += &self.nudge;
        // point += self.nudge;
        Ok(())
    }
    fn to_string(&self) -> String {
        format!(
            "nudge -p={} -xy={},{}",
            self.point, self.nudge.x, self.nudge.y
        )
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
// spawn spawner_name node node_name arg value
// spawn spawner_name graph iter-1011 segs-1017 enter-1015 brush-1014

#[derive(Debug)]
pub struct SpawnerCommandDispatch {
    spawners: HashMap<String, Vec<String>>,
    sub_commands: HashMap<&'static str, Box<Command>>,
}

impl SpawnerCommandDispatch {
    pub fn new() -> Self {
        let sub_commands = HashMap::new();
        Self {
            spawners: HashMap::new(),
            sub_commands,
        }
    }
    pub fn populate(mut self) -> Self {
        self.add_cmd(Box::new(SpawnerGraphCmd::new("blank".to_string(), "blank".to_string())));
        self
    }

    pub fn add_cmd(&mut self, cmd: Box<Command>) {
        self.sub_commands.insert(cmd.get_keyword(), cmd);
    }
}

impl Command for SpawnerCommandDispatch {
    fn get_keyword(&self) -> &'static str {
        "spawn"
    }
    // spawn spawner_name subcommand args....
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        let mut split = args.split_whitespace();
        let spawner_name = split.nth(1).ok_or(CmdError::NoSpawner(format!("missing spawner name {}", args)))?;

        if let Some(sp_cmd) = self.sub_commands.get(&spawner_name) {
            sp_cmd.parse_string(args)
        } else {
            Err(CmdError::NoSpawner(spawner_name.to_string()))
        }
    }
    fn execute(&mut self, state: &mut State) -> Result<(), &str> {
        // the dispatch should not have to execute anything, only dispatched commands will
        Ok(())
    }
    fn to_string(&self) -> String {
        "spawnerCommandDispatch ???".to_string()
    }
}

//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct SpawnerGraphCmd {
    spawner_name: String,
    graph: String,
}

impl SpawnerGraphCmd {
    pub fn new(spawner_name: String, graph: String) -> Self {
        Self{spawner_name, graph}
    }
}

impl Command for SpawnerGraphCmd {
    fn get_keyword(&self) -> &'static str {
        "graph"
    }
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        Err(CmdError::NotImplemented("spawnergraphcmd not implemented".to_string()))
    }
    fn execute(&mut self, state: &mut State) -> Result<(), &str> {
        // parse the
        Ok(())
    }
    fn to_string(&self) -> String {
        "spawngraphcmd ???".to_string()
    }
}
