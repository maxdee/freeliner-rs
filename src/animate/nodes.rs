use std::fmt::Debug;
use super::{RenderItem, Event};
use super::super::geometry::*;

/////////////////////////////////////////////////////////////////////////////////////
// #[derive(Debug)]
pub trait Node: Debug {
    fn do_thing(&self, event: Event, geom: &Data) -> Event;
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
}

impl Node for Iterate {
    fn do_thing(&self, mut event: Event, _geom: &Data) -> Event {
        let a = event.unit / self.count as f32;
        let i = 1.0 / self.count as f32;
        event.units = (0..self.count).map(|x| x as f32 * i + a).collect();
        event
    }
}

/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct GroupPicker {}

impl Node for GroupPicker {
    fn do_thing(&self, mut event: Event, _geom: &Data) -> Event {
        let mut group_list: Vec<(usize, f32)> = Vec::new();
        event
            .groups
            .iter()
            .for_each(|g| event.units.iter().for_each(|u| group_list.push((g.0, *u))));
        event.groups = group_list;
        event
    }
}

/////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct SelectSegs {}

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
            if !geom.groups[g.0].segments.is_empty() {
                geom.groups[g.0]
                    .segments
                    .iter()
                    .for_each(|s| seg_list.push((*s, g.1)));
            }
        });

        event.segments = seg_list;
        event
    }
}

//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Enterpolator {}

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
pub struct DrawDot {
    pub size: f32,
    // some sort of keyframes
}

impl Node for DrawDot {
    fn do_thing(&self, mut event: Event, _geom: &Data) -> Event {
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
pub struct SizeModulator {
    // some sort of keyframes
}

impl SizeModulator {
    fn modulate(&self, size: f32, unit: f32) -> f32 {
        size * if unit < 0.5 {unit*2.0} else {(1.0-unit)*2.0}
    }
}

impl Node for SizeModulator {
    fn do_thing(&self, mut event: Event, _geom: &Data) -> Event {
        {
            // let ev = &mut event;
            event.items.iter_mut().for_each(|item| {
                match item {
                    RenderItem::Dot {
                        ref mut size,
                        unit,
                        ..
                    } => *size = self.modulate(*size, *unit),
                    RenderItem::Line {
                        ref mut weight,
                        unit,
                        ..
                    } => *weight = self.modulate(*weight, *unit),
                    _ => (),
                }
            });
        }
        event
    }
}
//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct ExpandContract {
    // some sort of keyframes
}

// impl ExpandContract {
//     fn modulate(&self, size: f32, unit: f32) -> f32 {
//         size * if unit < 0.5 {unit*2.0} else {(1.0-unit)*2.0}
//     }
// }

impl Node for ExpandContract {
    fn do_thing(&self, mut event: Event, _geom: &Data) -> Event {
        {
            // let ev = &mut event;
            if !event.items.is_empty() {
                match event.items[0] {
                    RenderItem::Dot {
                        ref mut size,
                        unit,
                        ..
                    } => if unit < 0.5 {*size *= unit * 2.0},
                    _ => (),
                }
                let last = event.items.len()-1;
                match event.items[last] {
                    RenderItem::Dot {
                        ref mut size,
                        unit,
                        ..
                    } => if unit > 0.5 {*size *= (1.0 - unit) * 2.0},
                    _ => (),
                }
            }
            // for_each(|item| {
            // });
        }
        event
    }
}
