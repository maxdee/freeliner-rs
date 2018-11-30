use super::super::geometry::*;
use super::{Basket, RenderItem};
use std::fmt::Debug;

/////////////////////////////////////////////////////////////////////////////////////
pub trait Node: Debug {
    fn do_thing(&self, basket: Basket, geom: &Geometry) -> Basket;
    fn get_name(&self) -> &str;
    fn get_id(&self) -> usize;
}

/*
// Expose node parameters to UI!
pub struct SomeNode {
    count: u32,        // number box
    reverse: bool,     // checkbox
    fun_slider: f32,   // slider
}
*/

/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Iterate {
    pub count: u32,
    pub name: String,
    pub node_id: usize,
}

impl Node for Iterate {
    fn get_id(&self) -> usize {
        self.node_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn do_thing(&self, mut basket: Basket, _geom: &Geometry) -> Basket {
        let a = basket.unit / self.count as f32;
        let i = 1.0 / self.count as f32;
        basket.units = (0..self.count).map(|x| x as f32 * i + a).collect();
        basket
    }
}

/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct GroupPicker {
    pub name: String,
    pub node_id: usize,
}

impl Node for GroupPicker {
    fn get_id(&self) -> usize {
        self.node_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn do_thing(&self, mut basket: Basket, _geom: &Geometry) -> Basket {
        let mut group_list: Vec<(usize, f32)> = Vec::new();
        basket
            .groups
            .iter()
            .for_each(|g| basket.units.iter().for_each(|u| group_list.push((g.0, *u))));
        basket.groups = group_list;
        basket
    }
}

/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct SelectSegs {
    pub name: String,
    pub node_id: usize,
}

impl Node for SelectSegs {
    fn get_id(&self) -> usize {
        self.node_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn do_thing(&self, mut basket: Basket, geom: &Geometry) -> Basket {
        let mut seg_list: Vec<(usize, f32)> = Vec::new();
        // basket.groups.iter().map(|g| {
        //     geom.groups[g.0]
        //         .segments
        //         .iter()
        //         .map(|s| (s, g.1)).collect()
        // }).flatten().collect();
        basket.groups.iter().for_each(|g| {
            if geom.groups.len() > g.0{
                if !geom.groups[g.0].segments.is_empty() {
                    geom.groups[g.0]
                    .segments
                    .iter()
                    .for_each(|s| seg_list.push((*s, g.1)));
                }
            }
        });

        basket.segments = seg_list;
        basket
    }
}

//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Enterpolator {
    pub name: String,
    pub node_id: usize,
}

impl Node for Enterpolator {
    fn get_id(&self) -> usize {
        self.node_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn do_thing(&self, mut basket: Basket, geom: &Geometry) -> Basket {
        basket.points = basket
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
        basket
    }
}

//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct DrawDot {
    pub size: f32,
    // some sort of keyframes
    pub name: String,
    pub node_id: usize,

}

impl Node for DrawDot {
    fn get_id(&self) -> usize {
        self.node_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn do_thing(&self, mut basket: Basket, _geom: &Geometry) -> Basket {
        basket.items = basket
            .points
            .iter()
            .map(|p| RenderItem::Dot {
                pos: p.0.clone(),
                size: self.size,
                unit: p.1,
            })
            .collect();
        basket
    }
}

//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct SizeModulator {
    // some sort of keyframes
    pub name: String,
    pub node_id: usize,

}

impl SizeModulator {
    fn modulate(&self, size: f32, unit: f32) -> f32 {
        size * if unit < 0.5 {
            unit * 2.0
        } else {
            (1.0 - unit) * 2.0
        }
    }
}

impl Node for SizeModulator {
    fn get_id(&self) -> usize {
        self.node_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn do_thing(&self, mut basket: Basket, _geom: &Geometry) -> Basket {
        {
            // let ev = &mut basket;
            basket.items.iter_mut().for_each(|item| match item {
                RenderItem::Dot {
                    ref mut size, unit, ..
                } => *size = self.modulate(*size, *unit),
                RenderItem::Line {
                    ref mut weight,
                    unit,
                    ..
                } => *weight = self.modulate(*weight, *unit),
                _ => (),
            });
        }
        basket
    }
}
//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct ExpandContract {
    // some sort of keyframes
    pub name: String,
    pub node_id: usize,

}

// impl ExpandContract {
//     fn modulate(&self, size: f32, unit: f32) -> f32 {
//         size * if unit < 0.5 {unit*2.0} else {(1.0-unit)*2.0}
//     }
// }

impl Node for ExpandContract {
    fn get_id(&self) -> usize {
        self.node_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn do_thing(&self, mut basket: Basket, _geom: &Geometry) -> Basket {
        {
            // let ev = &mut basket;
            if !basket.items.is_empty() {
                match basket.items[0] {
                    RenderItem::Dot {
                        ref mut size, unit, ..
                    } => {
                        if unit < 0.5 {
                            *size *= unit * 2.0
                        }
                    }
                    RenderItem::Line {
                        ref mut weight,
                        unit,
                        ..
                    } => {
                        if unit < 0.5 {
                            *weight *= unit * 2.0
                        }
                    }
                    _ => (),
                }
                let last = basket.items.len() - 1;
                match basket.items[last] {
                    RenderItem::Dot {
                        ref mut size, unit, ..
                    } => {
                        if unit > 0.5 {
                            *size *= (1.0 - unit) * 2.0
                        }
                    }
                    _ => (),
                }
            }
            // for_each(|item| {
            // });
        }
        basket
    }
}
