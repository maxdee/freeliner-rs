pub use geometry::Point;

pub struct Input;
impl Input {
    fn new() -> Input {
        Input
    }

    fn mouse_pressed(&self, button: u8, pos: Point){
        println!("Pressed {} at {:?}", button, pos);
    }
    
}
