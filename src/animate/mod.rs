use super::geometry::*;
// use std::fmt::Debug;
pub mod nodes;
pub mod timer;
pub use self::nodes::*;
pub use self::timer::Timer;

// pub trait RenderItem {
//     // fn to_string(&self) -> String;
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Animator {
    // timer: Timer,
    // spawner: NodeTree,
    pub node_trees: Vec<NodeTree>,
    pub temp: f32,
}

impl Default for Animator {
    fn default() -> Self {
        let mut node_trees = Vec::new(); //
        let mut spwnr = NodeTree::new("aspawner".to_string());
        spwnr.add_geom(0).setup_nodes();
        // spwnr.add_geom(0).add_geom(1).setup_nodes();

        node_trees.push(spwnr);
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
            .flat_map(|sp| sp.run(lerp, geom))
            .collect()
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
            name: "iter".to_string(),
        }));
        self.nodes.push(Box::new(GroupPicker {
            name: "groups".to_string(),
        }));
        self.nodes.push(Box::new(SelectSegs {
            name: "segs".to_string(),
        }));
        self.nodes.push(Box::new(Enterpolator {
            name: "enter".to_string(),
        }));
        self.nodes.push(Box::new(DrawDot {
            size: 10.0,
            name: "brush".to_string(),
        }));
        // self.nodes.push(Box::new(SizeModulator {name: }));
        self.nodes.push(Box::new(ExpandContract {
            name: "expand".to_string(),
        }));
        self
    }
    pub fn run(&mut self, unit: f32, geom: &Geometry) -> Vec<RenderItem> {
        // make a vec
        //
        let starts: Vec<(usize, f32)> = self.groups.iter().map(|g| (*g, unit)).collect();

        if !geom.groups.is_empty() {
            return Vec::new();
        } else {

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
