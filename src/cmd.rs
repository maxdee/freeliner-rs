pub use geometry::*;
pub use super::State;
// in this command pattern, I would need to have indexes or keys to args

trait Command {
	// fn execute<T>(&self, args: T) -> Result<(), &str> where T: GenericData;
	// fn execute<T>(&self, args: T) -> Result<(), &str>;
	fn execute(&mut self) -> Result<(), &str>;
}


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
		let mut flag = false;
		if group.previous_point.is_some(){
		// if let Some(p_point) = &mut group.previous_point {
			// let p_point = group.previous_point.unwrap();
			let mut seg = StraightSegment::new(
				group.previous_point.as_ref().unwrap(),
				&self.new_point
			);
			group.segments.push(seg);
			group.previous_point.as_mut().unwrap().set(&self.new_point);
			// flag = true;
		} else {
			// let mut seg = StraightSegment::new(&self.new_point, &self.new_point);
			// group.segments.push(seg);
			group.previous_point = Some(Point::copy(&self.new_point));
		}
		// if !flag {
		// 	group.previous_point = Some(Point::copy(&self.new_point));
		// }

			// self.group.previous_point.as_mut().unwrap().set(&p);// = Some(p);

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
