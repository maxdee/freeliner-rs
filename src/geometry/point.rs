use std::ops::{AddAssign, MulAssign};

#[test]
fn test_point_lerp() {
    let point_a = Point::new(0.0, 0.0, 0.0);
    let point_b = Point::new(1.0, 1.0, 1.0);
    let result = Point::lerp(&point_a, &point_b, 0.5);
    let Point { x, y, z } = result;
    assert_eq!((0.5, 0.5, 0.5), (x, y, z));
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z }
    }

    pub fn new_2d(x: f32, y: f32) -> Self {
        Point { x, y, z: 0.0 }
    }

    pub fn copy(p: &Point) -> Self {
        Point {
            x: p.x,
            y: p.y,
            z: p.z,
        }
    }

    pub fn set(&mut self, loc: &Point) {
        self.x = loc.x;
        self.y = loc.y;
        self.x = loc.x;
    }

    pub fn dist(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    pub fn lerp(a: &Point, b: &Point, u: f32) -> Point {
        Point::new(lerp(a.x, b.x, u), lerp(a.y, b.y, u), lerp(a.z, b.z, u))
    }
}

impl<'a> AddAssign<&'a Point> for Point {
    fn add_assign(&mut self, other: &Point) {
        // why not &Point
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl<'a> MulAssign<&'a Point> for Point {
    fn mul_assign(&mut self, other: &Point) {
        // why not &Point
        *self = Point {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}
// impl MulAssign for Point {
//     type Input = f32;
//     fn mul_assign(&mut self, other: Point) { // why not &Point
//         *self = Point {
//             x: self.x * other.x,
//             y: self.y * other.y,
//             z: self.z * other.z,
//         };
//     }
// }

fn lerp(a: f32, b: f32, u: f32) -> f32 {
    a + (b - a) * u
    // if a < b {
    // } else {
    //     b + (a - b) * u
    // }
}
