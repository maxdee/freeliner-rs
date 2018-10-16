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

// access data
pub fn get_group(state: &mut State, index: usize) -> Result<&mut Group, &str> {
	if index < state.geom.groups.len() {
		Ok(&mut state.geom.groups[index])
	} else {
		Err("no such group")
	}
}

impl Command for AddPointCmd {
	// const NAME: &str = "addpoint";
	fn execute(&mut self, state: &mut State) -> Result<(), &str> {
		// let group = get_group(state, self.index)?;
		// do this better
		if self.index >= state.geom.groups.len() {
			return Err("no such group");
		}

		let group = &mut state.geom.groups[self.index];
		state.geom.points.push(Point::copy(&self.new_point));

		let new_index = state.geom.points.len()-1;

		if group.previous_point.is_some(){
			// push the new point, but if is snapped, then dont...
			// add a new segment
			state.geom.segs.push(Segment::new(
				group.previous_point.unwrap(),
				new_index
			));
			// add the segment to the group
			group.segments.push(state.geom.segs.len());
			// group.previous_point = Some(new_index);
		} else {
		}
		group.previous_point = Some(new_index);
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
				group.previous_point =
					Some(state.geom.segs[group.segments[group.segments.len()-1]].point_a);
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
		state.geom.points.push(Point::copy(&self.new_point));
		let new_index = state.geom.points.len()-1;
		group.previous_point = Some(new_index);
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

/////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct NudgePoint {
	// pub group: &mut SegmentGroup,
	pub index : usize,
	pub nudge: Point,
}

impl NudgePoint {
	pub fn new(index: usize, nudge : Point) -> Self {
		Self{index, nudge}
	}
}

impl Command for NudgePoint {
	fn execute(&mut self, state: &mut State) -> Result<(), &str> {
		// let mut point = &mut
		let i = self.index;
		state.geom.points[i] += &self.nudge;
		// point += self.nudge;
		Ok(())
	}
	fn get_name(&self) -> &'static str {
		"NudgePoint"
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
