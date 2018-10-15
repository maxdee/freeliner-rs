pub use geometry::*;
pub use super::State;
// in this command pattern, I would need to have indexes or keys to args

pub trait Command {
	// const NAME: &str;
	// fn execute<T>(&self, args: T) -> Result<(), &str> where T: GenericData;
	// fn execute<T>(&self, args: T) -> Result<(), &str>;
	fn execute(&mut self, state: &mut State) -> Result<(), &str>;
	fn get_name(&self) -> &'static str;
}

// pub enum Cmd {
// 	//AddPoint(AddPointCmd),
// 	// AddPoint(usize, Point), // index and new point
// 	AddPoint{index: usize, new_point: Point}, // anonymous struct
//
// }
//
// impl Cmd {
// 	fn execute(&mut self, state: &mut State) -> Result<(), &str>{
// 		Ok(())
// 	}
// }

/////////////////////////////////////////////////////////////////////////////////////////

// struct LoadGeomCmd<'a> {
// 	filepath: &'a str,
// 	group: &'a mut SegmentGroup,
// }
//
// impl<'a> LoadGeomCmd<'a> {
// 	fn new(filepath: &'a str, group: &'a mut SegmentGroup) -> Self{
// 		Self{ filepath, group}
// 	}
// }
//
// impl<'a> Command for LoadGeomCmd<'a> {
// 	fn execute(&mut self) -> Result<(), &str> {
// 		println!("load file {} into group {}", self.filepath, self.group.index);
// 		Err("load geom into group not implemented....")
// 	}
// }

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct AddPointCmd {
	// pub group: &mut SegmentGroup,
	pub index : usize,
	pub new_point: Point,
}

impl AddPointCmd {
	pub fn new(index: usize, new_point : Point) -> Self {
		Self{index, new_point}
	}
}

impl Command for AddPointCmd {
	// const NAME: &str = "addpoint";
	fn execute(&mut self, state: &mut State) -> Result<(), &str> {
		let group = &mut state.geom.groups[self.index];
		// do this better
		if group.previous_point.is_some(){
			state.geom.points.push(Point::copy(group.previous_point.as_ref().unwrap()));
			state.geom.points.push(Point::copy(&self.new_point));
			let index = state.geom.points.len()-1;
			// add a new segment
			state.geom.segs.push(Segment::new(index-1, index));
			// add the segment to the group
			group.segments.push(state.geom.segs.len());
			group.previous_point.as_mut().unwrap().set(&self.new_point);
		} else {
			group.previous_point = Some(Point::copy(&self.new_point));
		}
		Ok(())
	}
	fn get_name(&self) -> &'static str {
		"addpoint"
	}
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct RemovePoint {
	// pub group: &mut SegmentGroup,
	pub index : usize,
	pub new_point: Point,
}

impl RemovePoint {
	pub fn new(index: usize, new_point : Point) -> Self {
		Self{index, new_point}
	}
}

impl Command for RemovePoint {
	fn execute(&mut self, state: &mut State) -> Result<(), &str> {
		let group = &mut state.geom.groups[self.index];
		if group.segments.len() > 0 {
			group.segments.pop();
			if group.segments.len() == 0 {
				group.previous_point = None;
			} else {
				group.previous_point.as_mut().unwrap().set(
					&state.geom.points[
						state.geom.segs[group.segments[group.segments.len()-1]].point_b
					]
				);
			}
		}
		else {
			return Err("no points to remove");
		}
		Ok(())
	}
	fn get_name(&self) -> &'static str {
		"removepoint"
	}
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct BreakLine {
	// pub group: &mut SegmentGroup,
	pub index : usize,
	pub new_point: Point,
}

impl BreakLine {
	pub fn new(index: usize, new_point : Point) -> Self {
		Self{index, new_point}
	}
}

impl Command for BreakLine {
	fn execute(&mut self, state: &mut State) -> Result<(), &str> {
		let group = &mut state.geom.groups[self.index];
		group.previous_point.as_mut().unwrap().set(&self.new_point);
		Ok(())
	}
	fn get_name(&self) -> &'static str {
		"breakline"
	}
}

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct NewGroup{}

impl NewGroup {
	pub fn new() -> Self {
		Self{}
	}
}

impl Command for NewGroup {
	fn execute(&mut self, state: &mut State) -> Result<(), &str> {
		state.geom.groups.push(Group::new());
		Ok(())
	}
	fn get_name(&self) -> &'static str {
		"newgroup"
	}
}



// impl Command for AddPoint {
// 	// fn execute(&mut self, state: State) -> Result<(), &str> {
// 	// 	let &mut group = &mut state.geometric_data.groups[self.index];
// 	// 	if let Some(previous_point) = group.previous_point {
// 	// 		let mut seg = StraightSegment::new(&previous_point, &self.new_point);
// 	// 		group.segments.push(seg);
// 	// 	}
// 	// 	else {
// 	// 		let p = Point::copy(&self.new_point);
// 	// 		group.previous_point.unwrap().set(&p);// = Some(p);
// 	// 		// self.group.previous_point.as_mut().unwrap().set(&p);// = Some(p);
// 	// 	}
// 	// 	Ok(())
// 	// }
// }
