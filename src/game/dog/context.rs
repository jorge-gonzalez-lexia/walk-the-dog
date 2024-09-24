use crate::{
    engine::rect::{Point, Rect},
    game::{self},
};

pub const RUNNING_FRAMES: u8 = 60;
pub const DOG_FLOOR: i16 = game::HEIGHT - DOG_HEIGHT;
pub const DOG_HEIGHT: i16 = 89;

pub const JUMPING_FRAMES: u8 = 60;
pub const JUMP_SPEED: i16 = -25;

#[derive(Clone)]
pub struct DogContext {
    pub floor: i16,
    pub frame: u8,
    platform: Option<Rect>,
    pub position: Point,
    pub velocity: Point,
}

impl DogContext {
    pub fn new(frame: u8, position: Point, velocity: Point) -> Self {
        DogContext {
            floor: DOG_FLOOR,
            frame,
            platform: None,
            position,
            velocity,
        }
    }

    pub fn info(&self) -> String {
        format!(
            "pos={:?} v={:?} floor={:?}",
            self.position, self.velocity, self.floor
        )
    }

    pub fn floor(&self) -> i16 {
        if let Some(platform) = self.platform {
            platform.top() - DOG_HEIGHT
        } else {
            DOG_FLOOR
        }
    }

    pub fn set_floor(mut self, bottom: i16) -> Self {
        self.floor = bottom - DOG_HEIGHT;
        log!(
            "DogContext: set floor to {}",
            if self.floor == DOG_FLOOR {
                "Ground"
            } else {
                "Platform"
            }
        );

        self
    }

    pub fn reset_frame(mut self) -> Self {
        self.frame = 0;

        self
    }

    pub fn toggle_direction(mut self) -> Self {
        self.velocity.x *= -1;
        log!("Dog: toggled direction {:?}", self.info());

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

        if self.position.y > self.floor {
            self.position.y = self.floor;
        }

        self
    }
}
