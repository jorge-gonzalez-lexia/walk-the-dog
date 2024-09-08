use crate::engine::rect::Point;

pub struct DogContext {
    pub position: Point,
}

impl DogContext {
    pub fn new(position: Point) -> Self {
        DogContext { position }
    }
}
