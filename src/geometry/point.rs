
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        let mut p = Point {x: x, y: y, z: z};
        p
    }

    pub fn dist(&self, other: &Point) -> f64 {
        return ((self.x - other.x).powi(2) +
                (self.y - other.y).powi(2) +
                (self.z - other.z).powi(2)  ).sqrt()
    }

    pub fn lerp(a: &Point, b: &Point, u: f64) -> Point {
        let mut p = Point::new(
                                lerp(a.x, b.x, u),
                                lerp(a.y, b.y, u),
                                lerp(a.z, b.z, u)
                            );
        p
    }
}




fn lerp(a: f64, b: f64, u: f64) -> f64{
    if a < b {
        a + (b - a) * u
    }
    else {
        b + (b - a) * u
    }
}
