pub use geometry::*;
pub use super::State;
// in this command pattern, I would need to have indexes or keys to args

trait Command {
	// fn execute<T>(&self, args: T) -> Result<(), &str> where T: GenericData;
	// fn execute<T>(&self, args: T) -> Result<(), &str>;
	fn execute(&mut self) -> Result<(), &str>;
}

/////////////////////////////////////////////////////////////////////////////////////////

struct LoadGeomCmd<'a> {
	filepath: &'a str,
	group: &'a mut SegmentGroup,
}

impl<'a> LoadGeomCmd<'a> {
	fn new(filepath: &'a str, group: &'a mut SegmentGroup) -> Self{
		Self{ filepath, group}
	}
}

impl<'a> Command for LoadGeomCmd<'a> {
	fn execute(&mut self) -> Result<(), &str> {
		println!("load file {} into group {}", self.filepath, self.group.index);
		Err("load geom into group not implemented....")
	}
}

/////////////////////////////////////////////////////////////////////////////////////////

pub struct AddPoint {
	// pub group: &mut SegmentGroup,
	pub index : usize,
	pub new_point: Point,
}

impl AddPoint {
	pub fn new(index: usize, new_point : Point) -> Self {
		Self{index, new_point}
	}

	pub fn execute(&mut self, state: &mut State) -> Result<(), &str> {
		let group = &mut state.geometric_data.groups[self.index];
		if group.previous_point.is_some(){
			let mut seg = StraightSegment::new(
				group.previous_point.as_ref().unwrap(),
				&self.new_point
			);
			group.segments.push(seg);
			group.previous_point.as_mut().unwrap().set(&self.new_point);
		} else {
			group.previous_point = Some(Point::copy(&self.new_point));
		}
		Ok(())
	}
}

/////////////////////////////////////////////////////////////////////////////////////////

pub struct RemovePoint {
	// pub group: &mut SegmentGroup,
	pub index : usize,
	pub new_point: Point,
}

impl RemovePoint {
	pub fn new(index: usize, new_point : Point) -> Self {
		Self{index, new_point}
	}

	pub fn execute(&mut self, state: &mut State) -> Result<(), &str> {
		let group = &mut state.geometric_data.groups[self.index];
		// match group.segments.len() {
		// 	0 => (),
		// 	1 =>
		// 	n => Some(&self[n-1])
		// }
		if group.segments.len() > 0 {
			group.segments.pop();
			if group.segments.len() == 0 {
				group.previous_point = None;
			} else {
				group.previous_point.as_mut().unwrap().set(
					&group.segments[group.segments.len()-1].b
				);
			}
		}
		else {
			return Err("no points to remove");
		}
		Ok(())
	}
}

/////////////////////////////////////////////////////////////////////////////////////////

pub struct BreakLine {
	// pub group: &mut SegmentGroup,
	pub index : usize,
	pub new_point: Point,
}

impl BreakLine {
	pub fn new(index: usize, new_point : Point) -> Self {
		Self{index, new_point}
	}

	pub fn execute(&mut self, state: &mut State) -> Result<(), &str> {
		let group = &mut state.geometric_data.groups[self.index];
		group.previous_point.as_mut().unwrap().set(&self.new_point);
		Ok(())
	}
}

/////////////////////////////////////////////////////////////////////////////////////////

pub struct NewGroup{}

impl NewGroup {
	pub fn new() -> Self {
		Self{}
	}

	pub fn execute(&self, state: &mut State) -> Result<(), &str> {
		let mut sg = SegmentGroup::new(
			state.geometric_data.groups.len()
		);
		state.geometric_data.groups.push(sg);
		Ok(())
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
