use crate::engine::rect::Point;

#[derive(Clone)]
pub struct DogContext {
    pub frame: u8,
    pub position: Point,
}

impl DogContext {
    pub fn new(frame: u8, position: Point) -> Self {
        DogContext { frame, position }
    }

    pub fn update(mut self, frame_count: u8) -> Self {
        if self.frame < frame_count {
            self.frame += 1;
        } else {
            self.frame = 0
        }

        if self.position.x < 1000 {
            self.position.x += 4; // TODO velocity
        }

        self
    }
}
