use crate::{engine::rect::Point, game};

pub const RUNNING_FRAMES: u8 = 60;
pub const DOG_FLOOR: i16 = game::HEIGHT - DOG_HEIGHT;
const DOG_HEIGHT: i16 = 89;

pub const JUMPING_FRAMES: u8 = 60;
pub const JUMP_SPEED: i16 = -25;

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

    pub fn set_on(mut self, position: i16) -> Self {
        let position = position - DOG_HEIGHT;
        self.position.y = position;

        self
    }

    pub fn toggle_direction(mut self) -> Self {
        self.velocity.x *= -1;
        log!("Dog velocity {}", self.velocity.x);

        self
    }

    pub fn update(mut self, frame_count: u8) -> Self {
        if self.velocity.y < game::TERMINAL_VELOCITY {
            self.velocity.y += game::GRAVITY;
        }

        if self.frame < frame_count {
            self.frame += 1;
        } else {
            self.frame = 0
        }

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        if self.position.y > DOG_FLOOR {
            self.position.y = DOG_FLOOR;
        }

        self
    }
}
