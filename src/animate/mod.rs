use super::geometry::*;
// use std::fmt::Debug;
pub mod nodes;
pub mod timer;
pub use self::nodes::*;
pub use self::timer::Timer;
pub use std::collections::HashMap;
// pub trait RenderItem {
//     // fn to_string(&self) -> String;
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Animator {
    // timer: Timer,
    // spawner: NodeTree,
    pub node_trees: HashMap<String, NodeTree>,
    pub temp: f32,
}

impl Default for Animator {
    fn default() -> Self {
        let mut node_trees = HashMap::new(); //
        Self {
            // timer: Timer::default(),
            node_trees,
            temp: 0.0,
        }
    }
}

impl Animator {
    pub fn animate(&mut self, geom: &Geometry) -> Vec<RenderItem> {
        self.temp += 0.013;
        let lerp = self.temp.fract();

        self.node_trees
            .iter_mut()
            .flat_map(|sp| sp.1.run(lerp, geom))
            .collect()
    }
    pub fn populate(mut self) -> Self {
        let mut aaa = NodeTree::new("A".to_string());
        aaa.add_geom(0).setup_nodes();
        self.add_node_tree(aaa);
        self
    }
    pub fn add_node_tree(&mut self, tree: NodeTree) {
        self.node_trees.insert(tree.get_name().to_string(), tree);
    }
}

// Render items get interpreted by the renderer to draw stuff on screen
#[derive(Debug, Serialize, Deserialize)]
pub enum RenderItem {
    Dot {
        pos: Point,
        size: f32,
        unit: f32,
    },
    Square {
        pos: Point,
        size: f32,
        angle: f32,
        unit: f32,
    },
    Line {
        a: Point,
        b: Point,
        weight: f32,
        unit: f32,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeTree {
    // items: Vec<RenderItem>,
    groups: Vec<usize>,
    #[serde(skip)]
    pub nodes: Vec<Box<Node>>,
    life: f32,
    name: String,
}
//
// impl Default for NodeTree {
// }

// spawner spawns events and passes it through a set of nodes and collects RenderItems
impl NodeTree {
    fn new(name: String) -> Self {
        Self {
            groups: Vec::new(),
            nodes: Vec::new(),
            life: 0.0,
            name,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn add_geom(&mut self, g: usize) -> &mut Self {
        self.groups.push(g);
        self
    }
    pub fn setup_nodes(&mut self) -> &mut Self {
        self.nodes.push(Box::new(Iterate {
            count: 5,
            node_id: 1,
        }));
        self.nodes.push(Box::new(GroupPicker {
            node_id: 2,
        }));
        self.nodes.push(Box::new(SelectSegs {
            node_id: 3,
        }));
        self.nodes.push(Box::new(Enterpolator {
            node_id: 4,
        }));
        self.nodes.push(Box::new(DrawDot {
            size: 10.0,
            node_id: 5,
        }));
        self.nodes.push(Box::new(ExpandContract {
            node_id: 6,
        }));
        self
    }

    pub fn parse_graph_string(&mut self, graph_string: String) -> Result<(), String>{
        // println!("Helloo im node tree and i parse {}", graph_string);
        self.nodes.clear();
        let mut res = graph_string.split_whitespace()
            // should be map and collect results
            .for_each(|node| self.add_node_from_string(node));
            // .filter(|r| r.is_err());

        println!("adding node: {:?}", self.nodes);
        Ok(())
    }

    pub fn add_node_from_string(&mut self, node_string: &str) {// -> Result<(), String> {
        println!("{}", node_string);
        let mut split = node_string.split("-");
        let node_type = split.next().unwrap();
        let node_id = split.next().unwrap().parse::<usize>().unwrap();
        let mut node: Box<Node>;
        match node_type {
            "iter" => node = Box::new(Iterate{count: 5, node_id}),
            "groups" => node = Box::new(GroupPicker{node_id}),
            "segs" => node = Box::new(SelectSegs{node_id}),
            "enter" => node = Box::new(Enterpolator{node_id}),
            "brush" => node = Box::new(DrawDot{size: 20.0, node_id}),
            "sizemod" => node = Box::new(SizeModulator{node_id}),
            "expand_contract" => node = Box::new(ExpandContract{node_id}),
            _ => node = Box::new(Iterate{count: 5, node_id}),
        }
        self.nodes.push(node);
    }

    pub fn run(&mut self, unit: f32, geom: &Geometry) -> Vec<RenderItem> {
        // make a vec
        //
        let starts: Vec<(usize, f32)> = self.groups.iter().map(|g| (*g, unit)).collect();

        if geom.groups.is_empty() {
            return Vec::new();
        }


        let mut basket = Basket {
            mode: BasketModes::Loop,
            unit,
            groups: starts,
            segments: Vec::new(),
            points: Vec::new(),
            units: Vec::new(),
            items: Vec::new(),
        };
        // items.clear();
        // starts.iter().map(|start| {
        for node in self.nodes.iter() {
            basket = node.do_thing(basket, geom);
            // println!("//////////////{:?}//////////////////", node);
            // println!("event :{:#?}", event);
        }
        // }
        // items

        // // self.nodes.iter().fold(event, |n, ev| n.do_thing(ev, geom));
        basket.items
    }
}

#[derive(Debug)]
enum BasketModes {
    Loop,
    FadeOut,
    Single,
}

// pub struct Basket {
//     selected_groups: Vec<usize>, // the groups we render to
//
//     mode: EventModes,
// }

#[derive(Debug)]
pub struct Basket {
    mode: BasketModes,
    unit: f32,
    groups: Vec<(usize, f32)>, // good question
    segments: Vec<(usize, f32)>,
    points: Vec<(Point, f32)>,
    units: Vec<f32>,
    items: Vec<RenderItem>,
}
