use crate::engine::rect::Point;

pub const RUNNING_FRAMES: u8 = 60;

#[derive(Clone)]
pub struct DogContext {
    pub frame: u8,
    pub position: Point,
    pub velocity: Point,
}

impl DogContext {
    pub fn new(frame: u8, position: Point, velocity: Point) -> Self {
        DogContext {
            frame,
            position,
            velocity,
        }
    }

    pub fn reset_frame(mut self) -> Self {
        self.frame = 0;

        self
    }

    pub fn toggle_direction(mut self) -> Self {
        self.velocity.x *= -1;
        log!("Dog velocity {}", self.velocity.x);

        self
    }

    pub fn update(mut self, frame_count: u8) -> Self {
        if self.frame < frame_count {
            self.frame += 1;
        } else {
            self.frame = 0
        }

        self.position.x += self.velocity.x;

        self
    }
}
