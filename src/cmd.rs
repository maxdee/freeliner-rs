extern crate serde;
extern crate serde_json;

use super::animate::NodeTree;
pub use super::{Context, State};
pub use geometry::*;

// use self::serde_json::Error;

// use std::error;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

/////////////////////////////////////////////////////////////////////////////////////////

// pub use super::COMMAND_FACTORY;

// pub fn register_commands() {
//     COMMAND_FACTORY.add_cmd(Box::new(SaveStateCmd::default()));
// }

#[derive(Debug)]
pub enum CmdError {
    NoMatch,
    Malformed(String),
    NoCommand(String),
    NotImplemented(String),
    SomeError(String),
    FileError(),
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
        self.add_cmd(Box::new(NodeTreeCmdDispatch::default().populate()));

        self
    }

    pub fn add_cmd(&mut self, cmd: Box<Command>) {
        self.cmd_map.insert(cmd.get_keyword(), cmd);
    }

    pub fn add_subcommands(&mut self, _keyword: &str, _options: Vec<String>) {
        // self.cmd_map.get("spawn").add_node_tree(sp);
    }

    pub fn string_to_command(&self, string: String) -> Result<Box<Command>, CmdError> {
        // we need to skip the context name
        if let Some(keyword) = string.split_whitespace().nth(1) {
            if let Some(cmd) = self.cmd_map.get(keyword) {
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
        let mut ctx = state.get_context(self.get_context_name());
        match ctx {
            Some(c) => self.real_exec(c),
            _ => Err(CmdError::NoContext(self.get_context_name().to_string())),
        }
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
        Self {
            context_name: "*".to_string(),
            filepath,
        }
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
        match args.split_whitespace().nth(2) {
            Some(filepath) => Ok(Box::new(Self::new(filepath.to_string()))),
            _ => Err(CmdError::FileError()),
        }
    }

    fn execute(&mut self, state: &mut State) -> Result<(), CmdError> {
        let j = serde_json::to_vec(state).unwrap();
        let f = self.filepath.clone();
        let mut file = File::create(f).unwrap();
        file.write_all(&j).unwrap();
        Ok(())
    }

    fn real_exec(&mut self, context: &mut Context) -> Result<(), CmdError> {
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
    fn get_context_name(&self) -> &str {
        "*"
    }
    fn get_keyword(&self) -> &'static str {
        "loadstate"
    }
    // if supplied a filename or use default
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        match args.split_whitespace().nth(2) {
            Some(filepath) => Ok(Box::new(Self::new(filepath.to_string()))),
            _ => Err(CmdError::FileError()),
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
    /// Index of the point added
    pub index: Option<usize>,
    pub group: usize,
    pub new_point: Point,
}

impl AddPointCmd {
    pub fn new(context_name: String, group: usize, new_point: Point) -> Self {
        Self {
            context_name,
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
        let context_name = split
            .next()
            .ok_or_else(|| CmdError::Malformed(format!("missing context name : {}", args)))?
            .to_string();
        // skip the command name
        split.next();
        let group = split
            .next()
            .ok_or_else(|| CmdError::Malformed(format!("missing group arg : {}", args)))?
            .parse::<usize>()
            .ok()
            .ok_or(CmdError::Malformed(format!(
                "group arg not usize : {}",
                args
            )))?;

        let xpos = split
            .next()
            .ok_or_else(|| CmdError::Malformed(format!("missing xpos : {}", args)))?
            .parse::<f32>()
            .ok()
            .ok_or(CmdError::Malformed(format!(
                "xpos not parseable : {}",
                args
            )))?;

        let ypos = split
            .next()
            .ok_or_else(|| CmdError::Malformed(format!("missing ypos : {}", args)))?
            .parse::<f32>()
            .ok()
            .ok_or(CmdError::Malformed(format!(
                "ypos not parseable : {}",
                args
            )))?;

        let zpos = split.next().unwrap_or("0.0").parse::<f32>().unwrap_or(0.0);

        Ok(Box::new(Self::new(
            context_name,
            group,
            Point::new(xpos, ypos, zpos),
        )))
    }
    // const NAME: &str = "addpoint";
    fn real_exec(&mut self, context: &mut Context) -> Result<(), CmdError> {
        // let group = get_group(state, self.index)?;
        // do this better
        if self.group >= context.geometry.groups.len() {
            return Err(CmdError::NoExecute("no such group".to_string())); //format!("no such group {}", self.to_string()));
        }

        let group = &mut context.geometry.groups[self.group];
        context.geometry.points.push(Point::copy(&self.new_point));

        let new_index = context.geometry.points.len() - 1;
        self.index = Some(new_index);

        if group.previous_point.is_some() {
            // push the new point, but if is snapped, then dont...
            // add a new segment
            context
                .geometry
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
        Self {
            context_name,
            group,
        }
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
        let mut split = args.split_whitespace();
        let context_name = split
            .next()
            .ok_or_else(|| CmdError::Malformed(format!("missing context name : {}", args)))?
            .to_string();

        split.next();

        if let Some(group) = split.next() {
            if let Ok(index) = group.parse::<usize>() {
                Ok(Box::new(Self::new(context_name, index)))
            } else {
                Err(CmdError::Malformed(args.to_string()))
            }
        } else {
            Err(CmdError::Malformed(args.to_string()))
        }
    }

    fn real_exec(&mut self, context: &mut Context) -> Result<(), CmdError> {
        let group = context
            .geometry
            .groups
            .get_mut(self.group)
            .ok_or(CmdError::NoExecute("no such group".to_string()))?;

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
        Self {
            context_name,
            group,
            new_point,
        }
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
    fn real_exec(&mut self, context: &mut Context) -> Result<(), CmdError> {
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
        Self { context_name }
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
        match args.split_whitespace().next() {
            Some(name) => Ok(Box::new(Self::new(name.to_string()))),
            None => Err(CmdError::NoContext("cant find context name".to_string())),
        }
    }
    fn real_exec(&mut self, context: &mut Context) -> Result<(), CmdError> {
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
        Self {
            context_name,
            point,
            nudge,
        }
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
    fn real_exec(&mut self, context: &mut Context) -> Result<(), CmdError> {
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
// default_ctx A node node_name arg value
// default A graph iter-1011 segs-1017 enter-1015 brush-1014

#[derive(Debug, Default)]
pub struct NodeTreeCmdDispatch {
    context_name: String,
    trees_n_nodes: HashMap<String, Vec<String>>,
    sub_commands: HashMap<&'static str, Box<Command>>,
}

impl NodeTreeCmdDispatch {
    pub fn new(context_name: String) -> Self {
        let sub_commands = HashMap::new();
        Self {
            context_name,
            trees_n_nodes: HashMap::new(),
            sub_commands,
        }
    }
    // add comands for individual nodes
    pub fn populate(mut self) -> Self {
        self.add_cmd(Box::new(NodeTreeCmd::new(
            "default".to_string(),
            "blank".to_string(),
            "blank".to_string(),
        )));
        self.add_cmd(Box::new(NodeParamCmd::new(
            "default".to_string(),
            "blank".to_string(),
            0,
            "blank".to_string(),
            "blank".to_string(),
        )));
        self
    }
    pub fn add_cmd(&mut self, cmd: Box<Command>) {
        self.sub_commands.insert(cmd.get_keyword(), cmd);
    }

    pub fn add_node_tree(&mut self, node_tree: &NodeTree) {
        let nodes = node_tree
            .nodes
            .iter()
            .map(|node| String::from(node.get_name()))
            .collect();
        self.trees_n_nodes
            .insert(String::from(node_tree.get_name()), nodes);
    }
}

impl Command for NodeTreeCmdDispatch {
    fn get_context_name(&self) -> &str {
        &self.context_name
    }
    fn get_keyword(&self) -> &'static str {
        "tree"
    }
    // default tree A subcommand args....

    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        let mut split = args.split_whitespace();
        // consume defualt tree
        split.next();
        split.next();
        let node_tree_name = split
            .next()
            .ok_or_else(|| CmdError::Malformed(format!("missing node_tree name {}", args)))?;

        let cmd_name = split.next().ok_or_else(|| {
            CmdError::Malformed(format!("missing node_tree command name {}", args))
        })?;

        if let Some(sp_cmd) = self.sub_commands.get(&cmd_name) {
            sp_cmd.parse_string(args)
        } else {
            Err(CmdError::Malformed(cmd_name.to_string()))
        }
    }

    fn real_exec(&mut self, context: &mut Context) -> Result<(), CmdError> {
        // the dispatch should not have to execute anything, only dispatched commands will
        Ok(())
    }

    fn to_string(&self) -> String {
        "spawnerCommandDispatch ???".to_string()
    }
}

//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct NodeTreeCmd {
    context_name: String,
    node_tree_name: String,
    graph: String,
}

impl NodeTreeCmd {
    pub fn new(context_name: String, node_tree_name: String, graph: String) -> Self {
        Self {
            context_name,
            node_tree_name,
            graph,
        }
    }
}

impl Command for NodeTreeCmd {
    fn get_context_name(&self) -> &str {
        &self.context_name
    }
    fn get_keyword(&self) -> &'static str {
        "graph"
    }
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        let mut split = args.split_whitespace();
        let context_name = split
            .next()
            .ok_or_else(|| CmdError::Malformed(format!("missing context name : {}", args)))?
            .to_string();
        // consume "tree"
        split.next();
        let node_tree_name = split
            .next()
            .ok_or_else(|| CmdError::Malformed(format!("missing tree name : {}", args)))?
            .to_string();
        // skip the graph
        split.next();
        let mut graph = split.fold(String::new(), |mut graph, node| {
            graph.push_str(node);
            graph.push(' ');
            graph
        });

        Ok(Box::new(NodeTreeCmd::new(
            context_name,
            node_tree_name,
            graph,
        )))
    }

    fn real_exec(&mut self, context: &mut Context) -> Result<(), CmdError> {
        // parse the
        if let Some(tree) = context.animator.node_trees.get_mut(&self.node_tree_name) {
            tree.parse_graph_string(self.graph.clone());
            Ok(())
        } else {
            Err(CmdError::NoExecute(format!(
                "no graph tree found {}, perhaps create {}",
                self.node_tree_name, self.node_tree_name
            )))
        }
    }
    fn to_string(&self) -> String {
        "node tree making".to_string()
    }
}

//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct NodeParamCmd {
    context_name: String,
    tree_name: String,
    node_id: usize,
    param_name: String,
    param_val: String,

}

impl NodeParamCmd {
    pub fn new(context_name: String, tree_name: String, node_id: usize, param_name: String, param_val: String) -> Self {
        Self {
            context_name,
            tree_name,
            node_id,
            param_name,
            param_val,
        }
    }
}

impl Command for NodeParamCmd {
    fn get_context_name(&self) -> &str {
        &self.context_name
    }
    fn get_keyword(&self) -> &'static str {
        "node"
    }
    // default tree A node brush-1011 size 42.0
    fn parse_string(&self, args: &str) -> Result<Box<Command>, CmdError> {
        let mut split = args.split_whitespace();
        let context_name = split
            .next()
            .ok_or_else(|| CmdError::Malformed(format!("missing context name : {}", args)))?
            .to_string();
        // consume "tree"
        split.next();
        let tree_name = split
            .next()
            .ok_or_else(|| CmdError::Malformed(format!("missing tree name : {}", args)))?
            .to_string();
        // skip the node keyword
        split.next();
        let node_id = split
            .next()
            .ok_or_else(|| CmdError::Malformed(format!("missing node name : {}", args)))?
            .split("-")
            .nth(1)
            .ok_or_else(|| CmdError::Malformed(format!("missing node id name : {}", args)))?
            .parse::<usize>()
            .ok()
            .ok_or_else(|| CmdError::Malformed(format!("cant parse node id: {}", args)))?;

        let param_name = split.next()
            .ok_or_else(|| CmdError::Malformed(format!("missing param name : {}", args)))?
            .to_string();

        let param_val = split.next()
            .ok_or_else(|| CmdError::Malformed(format!("missing param val : {}", args)))?
            .to_string();

        let mut cmd = Box::new(NodeParamCmd::new(
            context_name,
            tree_name,
            node_id,
            param_name,
            param_val,
        ));
        Ok(cmd)
    }

    fn real_exec(&mut self, context: &mut Context) -> Result<(), CmdError> {
        // fetch the tree
        if let Some(tree) = context.animator.node_trees.get_mut(&self.tree_name) {
            // fetch the node
            if let Ok(node) = tree.get_node(self.node_id) {
                node.set_param(&self.param_name, &self.param_val);
            } else {
                // Err(CmdError::NoExecute("could not get node".to_string())
            }
            Ok(())
        } else {
            Err(CmdError::NoExecute(format!(
                "no graph tree found {}, perhaps create {}",
                self.tree_name, self.tree_name
            )))
        }
    }
    fn to_string(&self) -> String {
        "node tree making".to_string()
    }
}
