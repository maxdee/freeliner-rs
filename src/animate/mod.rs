use super::geometry::*;
use std::fmt::Debug;
pub mod timer;
pub mod nodes;
pub use self::nodes::*;
pub use self::timer::Timer;


// pub trait RenderItem {
//     // fn to_string(&self) -> String;
// }

#[derive(Debug)]
pub struct Animator {
    // timer: Timer,
    spawner: Spawner,
    temp: f32,
}

impl Default for Animator {
    fn default() -> Self {
        let mut spawner = Spawner::default();
        spawner.add_geom(0).add_geom(1).setup_nodes();
        Self {
            // timer: Timer::default(),
            spawner,
            temp: 0.0,
        }
    }
}

impl Animator {
    pub fn animate(&mut self, geom: &Data) -> Vec<RenderItem> {
        self.temp += 0.013;
        self.temp = self.temp.fract();
        self.spawner.run(self.temp, geom)
    }
}

// Render items get interpreted by the renderer to draw stuff on screen
#[derive(Debug)]
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

#[derive(Debug)]
pub struct Spawner {
    items: Vec<RenderItem>,
    groups: Vec<usize>,
    nodes: Vec<Box<Node>>,
    life: f32,
}

impl Default for Spawner {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            groups: Vec::new(),
            nodes: Vec::new(),
            life: 0.0,
        }
    }
}

// spawner spawns events and passes it through a set of nodes and collects RenderItems
impl Spawner {
    pub fn add_geom(&mut self, g: usize) -> &mut Self {
        self.groups.push(g);
        self
    }
    pub fn setup_nodes(&mut self) -> &mut Self {
        self.nodes.push(Box::new(Iterate { count: 5 }));
        self.nodes.push(Box::new(GroupPicker {}));
        self.nodes.push(Box::new(SelectSegs {}));
        self.nodes.push(Box::new(Enterpolator {}));
        self.nodes.push(Box::new(DrawDot { size: 10.0 }));
        // self.nodes.push(Box::new(SizeModulator {}));
        self.nodes.push(Box::new(ExpandContract {}));
        self
    }
    pub fn run(&mut self, unit: f32, geom: &Data) -> Vec<RenderItem> {
        let starts = geom.groups.iter().map(|g| (g.index, unit)).collect();
        // if geom.groups.len() < 1 {
        //     return Vec::new();
        // }

        let mut event = Event {
            unit,
            groups: starts,
            segments: Vec::new(),
            points: Vec::new(),
            units: Vec::new(),
            items: Vec::new(),
        };

        for node in self.nodes.iter() {
            event = node.do_thing(event, geom);
            // println!("//////////////{:?}//////////////////", node);
            // println!("event :{:#?}", event);
        }
        // self.nodes.iter().fold(event, |n, ev| n.do_thing(ev, geom));
        event.items
    }
}

#[derive(Debug)]
pub struct Event {
    unit: f32,
    groups: Vec<(usize, f32)>,
    segments: Vec<(usize, f32)>,
    points: Vec<(Point, f32)>,
    units: Vec<f32>,
    items: Vec<RenderItem>,
}
