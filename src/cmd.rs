extern crate serde;
extern crate serde_json;

pub use super::{State, Context};
use super::animate::Spawner;
pub use geometry::*;

// use self::serde_json::Error;

// use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

/////////////////////////////////////////////////////////////////////////////////////////

pub use super::COMMAND_FACTORY;

pub fn register_commands() {
    COMMAND_FACTORY.add_cmd(Box::new(SaveStateCmd::default()));
}

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
    NoContext(String),
    NoExecute(String),
}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CmdError::NoMatch => f.write_str("not a match"),
            CmdError::NoCommand(ref string) => f.write_str(&format!("no command : {:?}", string)),
            CmdError::NotImplemented(ref string) => f.write_str(string),
            CmdError::NoExecute(ref string) => f.write_str(&format!("no exec : {}", string)),
            CmdError::NoContext(ref string) => f.write_str(&format!("no context : {}", string)),
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
        self.add_cmd(Box::new(SpawnerCommandDispatch::default()));

        self
    }

    pub fn add_cmd(&mut self, cmd: Box<Command>) {
        self.cmd_map.insert(cmd.get_keyword(), cmd);
    }

    pub fn add_subcommands(&mut self, _keyword: &str, _options: Vec<String>) {
        // self.cmd_map.get("spawn").add_spawner(sp);
    }

    pub fn string_to_command(&self, string: String) -> Result<Box<Command>, CmdError> {
        // we need to skip the context name
        if let Some(keyword) = string.split_whitespace().nth(1) {
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
    fn get_keyword(&self) -> &'static str;
    fn to_string(&self) -> String;
    fn get_context_name(&self) -> &str;
    fn real_exec(&mut self, context: &mut Context) -> Result<(), CmdError>;
    fn execute(&mut self, state: &mut State) -> Result<(), CmdError> {
        let mut ctxs = state.get_contexts(self.get_context_name());
        let results: Vec<Result<(), CmdError>> = ctxs.iter()
            .map(|ct| {
                match ct {
                    Some(c) => self.real_exec(*c),
                    _ => Err(CmdError::NoContext(self.get_context_name().to_string())),
                }
            })
            .collect();
        results[0]
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct SaveStateCmd {
    context_name: String,
    pub filepath: String,
}

impl SaveStateCmd {
    pub fn new(filepath: String) -> Self {
        Self{ context_name: "*".to_string(), filepath }
    }
}

impl Command for SaveStateCmd {
    fn get_context_name(&self) -> &str {
        &self.context_name
    }
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

    fn execute(&mut self, state: &mut State) -> Result<(), CmdError> {
        let j = serde_json::to_vec(state).unwrap();
        let f = self.filepath.clone();
        let mut file = File::create(f).unwrap();
        file.write_all(&j).unwrap();
        Ok(())
    }
    fn real_exec(&mut self, context : &mut Context) -> Result<(), CmdError> {
        // let j = serde_json::to_vec(state).unwrap();
        // let f = self.filepath.clone();
        // let mut file = File::create(f).unwrap();
        // file.write_all(&j).unwrap();
        Ok(())
    }

    fn to_string(&self) -> String {
        format!("savestate {}", self.filepath,)
    }
}


/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct LoadStateCmd {
    context_name: String,
    pub filepath: String,
}

impl LoadStateCmd {
    pub fn new(context_name: String, filepath: String) -> Self {
        Self{ context_name, filepath }
    }
}

impl Command for LoadStateCmd {
    fn get_context_name(&self) -> &str {
        &self.context_name
    }
    fn get_keyword(&self) -> &'static str {
        "loadstate"
    }
    // if supplied a filename or use default
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        let mut split = args.split_whitespace();
        match split.next() {
            Some(filepath) => {
                Ok(Box::new(Self::new(filepath.to_string())))
            },
            _ => Err(CmdError::NoFile),
        }
    }

    fn execute(&mut self, state: &mut State) -> Result<(), CmdError> {
        let f = self.filepath.clone();
        let mut file = File::open(f).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        *state = serde_json::from_str(&contents).unwrap();
        Ok(())
    }
    fn real_exec(&mut self, context: &mut Context) -> Result<(), CmdError> {
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
    context_name: String,
    /// Context on which to act
    pub context: String,
    /// Index of the point added
    pub index: Option<usize>,
    pub group: usize,
    pub new_point: Point,
}

impl AddPointCmd {
    pub fn new(context_name: String, context: String, group: usize, new_point: Point) -> Self {
        Self{ context_name,
            context,
            index: None,
            group,
            new_point,
        }
    }
}

impl Command for AddPointCmd {
    fn get_context_name(&self) -> &str {
        &self.context_name
    }
    fn get_keyword(&self) -> &'static str {
        "addpoint"
    }
    // addpoint -g=3 -xy=20,30
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        let mut split = args.split_whitespace();
        split.next();
        let group = split.next()
            .ok_or_else(|| CmdError::Malformed(format!("missing group arg : {}", args)))?
            .parse::<usize>()
            .ok()
            .ok_or(CmdError::Malformed(format!("group arg not usize : {}", args)))?;

        let xpos = split.next()
            .ok_or_else(|| CmdError::Malformed(format!("missing xpos : {}", args)))?
            .parse::<f32>()
            .ok()
            .ok_or(CmdError::Malformed(format!("xpos not parseable : {}", args)))?;

        let ypos = split.next()
            .ok_or_else(|| CmdError::Malformed(format!("missing ypos : {}", args)))?
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
    fn real_exec(&mut self, context : &mut Context) -> Result<(), CmdError> {
        // let group = get_group(state, self.index)?;
        // do this better
        if self.group >= context.geometry.groups.len() {
            return Err("no such group"); //format!("no such group {}", self.to_string()));
        }

        let group = &mut context.geometry.groups[self.group];
        context.geometry.points.push(Point::copy(&self.new_point));

        let new_index = context.geometry.points.len() - 1;
        self.index = Some(new_index);

        if group.previous_point.is_some() {
            // push the new point, but if is snapped, then dont...
            // add a new segment
            state
                .geom
                .segs
                .push(Segment::new(group.previous_point.unwrap(), new_index));
            // add the segment to the group
            group.segments.push(context.geometry.segs.len());
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
    context_name: String,
    // pub group: &mut SegmentGroup,
    pub group: usize,
}

impl RemovePointCmd {
    pub fn new(context_name: String, group: usize) -> Self {
        Self{ context_name, group }
    }
}

impl Command for RemovePointCmd {
    fn get_context_name(&self) -> &str {
        &self.context_name
    }
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

    fn real_exec(&mut self, context : &mut Context) -> Result<(), CmdError> {
        let group = context
            .geometry
            .groups
            .get_mut(self.group)
            .ok_or("no such group")?;

        if !group.segments.is_empty() {
            group.segments.pop();
            if group.segments.is_empty() {
                group.previous_point = None;
            } else {
                group.previous_point =
                    Some(context.geometry.segs[group.segments[group.segments.len() - 1]].point_a);
            }
        } else {
            return Err(CmdError::NoExecute("no points to remove".to_string()));
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
    context_name: String,
    // pub group: &mut SegmentGroup,
    pub group: usize,
    pub new_point: Point,
}

impl BreakLineCmd {
    pub fn new(context_name: String, group: usize, new_point: Point) -> Self {
        Self{ context_name, group, new_point }
    }
}

impl Command for BreakLineCmd {
    fn get_context_name(&self) -> &str {
        &self.context_name
    }
    fn get_keyword(&self) -> &'static str {
        "breakline"
    }
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        Err(CmdError::NotImplemented(args.to_string()))
    }
    fn real_exec(&mut self, context : &mut Context) -> Result<(), CmdError> {
        let group = &mut context.geometry.groups[self.group];
        context.geometry.points.push(Point::copy(&self.new_point));
        let new_index = context.geometry.points.len() - 1;
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
pub struct NewGroupCmd {
    pub context_name: String,
}

impl NewGroupCmd {
    pub fn new(context_name: String) -> Self {
        Self{ context_name }
    }
}

impl Command for NewGroupCmd {
    fn get_context_name(&self) -> &str {
        &self.context_name
    }
    fn get_keyword(&self) -> &'static str {
        "newgroup"
    }
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        match args.split_whitespace().nth(1) {
            Some(name) => Ok(Box::new(Self::new(name.to_string()))),
            None => Err(CmdError::NoContext("cant find context name".to_string())),
        }
    }
    fn real_exec(&mut self, context : &mut Context) -> Result<(), CmdError> {
        let i = context.geometry.groups.len();
        context.geometry.groups.push(Group::new(i));
        Ok(())
    }
    fn to_string(&self) -> String {
        String::from("newgroup")
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct NudgePointCmd {
    context_name: String,
    // pub group: &mut SegmentGroup,
    pub point: usize,
    pub nudge: Point,
}

impl NudgePointCmd {
    pub fn new(context_name: String, point: usize, nudge: Point) -> Self {
        Self{context_name, point, nudge }
    }
}

impl Command for NudgePointCmd {
    fn get_context_name(&self) -> &str {
        &self.context_name
    }
    fn get_keyword(&self) -> &'static str {
        "nudgepoint"
    }
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        Err(CmdError::NotImplemented(args.to_string()))
    }
    fn real_exec(&mut self, context : &mut Context) -> Result<(), CmdError> {
        // let mut point = &mut
        let i = self.point;
        context.geometry.points[i] += &self.nudge;
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

#[derive(Debug, Default)]
pub struct SpawnerCommandDispatch {
    context_name: String,
    spawners_n_nodes: HashMap<String, Vec<String>>,
    sub_commands: HashMap<&'static str, Box<Command>>,
}

impl SpawnerCommandDispatch {
    pub fn new(context_name: String) -> Self {
        let sub_commands = HashMap::new();
        Self {
            context_name,
            spawners_n_nodes: HashMap::new(),
            sub_commands,
        }
    }

    pub fn populate(mut self) -> Self {
        self.add_cmd(Box::new(SpawnerGraphCmd::new("blank".to_string(), "blank".to_string())));
        self
    }

    pub fn add_spawner(&mut self, spawner: &Spawner) {
        let nodes = spawner
            .nodes.iter()
            .map(|node| String::from(node.get_name()))
            .collect();
        self.spawners_n_nodes.insert(String::from(spawner.get_name()), nodes);
    }
    pub fn add_cmd(&mut self, cmd: Box<Command>) {
        self.sub_commands.insert(cmd.get_keyword(), cmd);
    }
}

impl Command for SpawnerCommandDispatch {
    fn get_context_name(&self) -> &str {
        &self.context_name
    }
    fn get_keyword(&self) -> &'static str {
        "spawn"
    }
    // spawn spawner_name subcommand args....
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        let mut split = args.split_whitespace();
        let spawner_name = split.nth(1).ok_or_else(|| CmdError::NoSpawner(format!("missing spawner name {}", args)))?;

        if let Some(sp_cmd) = self.sub_commands.get(&spawner_name) {
            sp_cmd.parse_string(args)
        } else {
            Err(CmdError::NoSpawner(spawner_name.to_string()))
        }
    }

    fn real_exec(&mut self, context : &mut Context) -> Result<(), CmdError> {
        // the dispatch should not have to execute anything, only dispatched commands will
        Ok(())
    }

    fn to_string(&self) -> String {
        "spawnerCommandDispatch ???".to_string()
    }
}

//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct SpawnerGraphCmd {
    context_name: String,
    spawner_name: String,
    graph: String,
}

impl SpawnerGraphCmd {
    pub fn new(context_name: String, spawner_name: String, graph: String) -> Self {
        Self{context_name, spawner_name, graph}
    }
}

impl Command for SpawnerGraphCmd {
    fn get_context_name(&self) -> &str {
        &self.context_name
    }
    fn get_keyword(&self) -> &'static str {
        "graph"
    }
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        Err(CmdError::NotImplemented("spawnergraphcmd not implemented".to_string()))
    }
    fn real_exec(&mut self, context : &mut Context) -> Result<(), CmdError> {
        // parse the
        Ok(())
    }
    fn to_string(&self) -> String {
        "spawngraphcmd ???".to_string()
    }
}
