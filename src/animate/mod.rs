use super::geometry::*;
use std::fmt::Debug;
pub mod timer;
pub use self::timer::Timer;

// pub trait RenderItem {
//     // fn to_string(&self) -> String;
// }

#[derive(Debug)]
pub struct Animator {
    timer: Timer,
    spawner: Spawner,
    temp: f32,
}

impl Animator {
    pub fn new() -> Self {
        let mut spawner = Spawner::new();
        spawner.add_geom(0).add_geom(1).setup_nodes();
        Self {
            timer: Timer::new(),
            spawner,
            temp: 0.0,
        }
    }
    pub fn animate(&mut self, geom: &Data) -> Vec<RenderItem> {
        self.temp += 0.01;
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

// spawner spawns events and passes it through a set of nodes and collects RenderItems
impl Spawner {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            groups: Vec::new(),
            nodes: Vec::new(),
            life: 0.0,
        }
    }
    pub fn add_geom(&mut self, g: usize) -> &mut Self {
        self.groups.push(g);
        self
    }
    pub fn setup_nodes(&mut self) -> &mut Self {
        self.nodes.push(Box::new(Iterate { count: 4 }));
        self.nodes.push(Box::new(SelectSegs {}));
        self.nodes.push(Box::new(Enterpolator {}));
        self.nodes.push(Box::new(DrawDot { size: 10.0 }));
        self.nodes.push(Box::new(SizeModulator{}));

        self
    }
    pub fn run(&mut self, unit: f32, geom: &Data) -> Vec<RenderItem> {
        if geom.groups.len() < 2 {
            return Vec::new();
        }
        let mut event = Event {
            groups: vec![(0, unit), (1, unit)],
            segments: Vec::new(),
            points: Vec::new(),
            unit,
            units: Vec::new(),
            items: Vec::new(),
        };

        // println!("unit :{}", unit);
        for node in self.nodes.iter() {
            // println!("//////////////{:?}//////////////////", node);
            event = node.do_thing(event, geom);
            // println!("event :{:#?}", event);
        }
        // self.nodes.iter().fold(event, |n, ev| n.do_thing(ev, geom));
        event.items
    }
}

#[derive(Debug)]
pub struct Event {
    groups: Vec<(usize, f32)>,
    segments: Vec<(usize, f32)>,
    points: Vec<(Point, f32)>,
    unit: f32,
    units: Vec<f32>,
    items: Vec<RenderItem>,
}

/////////////////////////////////////////////////////////////////////////////////////
// #[derive(Debug)]
trait Node: Debug {
    fn do_thing(&self, event: Event, geom: &Data) -> Event;
}
/*
// Expose node parameters to UI!
struct SomeNode {
    count: u32,        // number box
    reverse: bool,     // checkbox
    fun_slider: f32,   // slider
}
*/

/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Iterate {
    count: u32,
}

impl Node for Iterate {
    fn do_thing(&self, mut event: Event, geom: &Data) -> Event {
        let a = event.unit / self.count as f32;
        let i = 1.0 / self.count as f32;
        event.units = (0..self.count).map(|x| x as f32 * i + a).collect();
        event
    }
}

/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct SelectSegs {}

impl Node for SelectSegs {
    fn do_thing(&self, mut event: Event, geom: &Data) -> Event {
        let mut seg_list: Vec<(usize, f32)> = Vec::new();
        // event.groups.iter().map(|g| {
        //     geom.groups[g.0]
        //         .segments
        //         .iter()
        //         .map(|s| (s, g.1)).collect()
        // }).flatten().collect();
        event.groups.iter().for_each(|g| {
            if geom.groups[g.0].segments.len() > 0 {
                geom.groups[g.0]
                    .segments
                    .iter()
                    .for_each(|s| seg_list.push((s.clone(), g.1)));
            }
        });

        event.segments = seg_list;
        event
    }
}

//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Enterpolator {}

impl Node for Enterpolator {
    fn do_thing(&self, mut event: Event, geom: &Data) -> Event {
        event.points = event
            .segments
            .iter()
            .map(|s| {
                let seg = &geom.segs[s.0 - 1];
                // will need to pack angles and stuff also
                (
                    Point::lerp(&geom.points[seg.point_a], &geom.points[seg.point_b], s.1),
                    s.1,
                )
            })
            .collect();
        event
    }
}

//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct DrawDot {
    size: f32,
    // some sort of keyframes
}

impl Node for DrawDot {
    fn do_thing(&self, mut event: Event, geom: &Data) -> Event {
        event.items = event
            .points
            .iter()
            .map(|p| RenderItem::Dot {
                pos: p.0.clone(),
                size: self.size,
                unit: p.1,
            })
            .collect();
        event
    }
}

//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct SizeModulator {
    // some sort of keyframes
}

impl Node for SizeModulator {
    fn do_thing(&self, mut event: Event, geom: &Data) -> Event {
        event.items.iter().map(|item| {
            match item {
                RenderItem::Dot { pos, mut size, unit } => size *= unit,
                // RenderItem::Line => item.weight *= item.unit,
                _ => (),
            }
        });
        event
    }
}
