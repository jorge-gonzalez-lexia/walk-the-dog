use crate::{
    engine::rect::Point,
    game::{self},
};

pub const RUNNING_FRAMES: u8 = 60;
pub const DOG_GROUND: i16 = game::HEIGHT - DOG_HEIGHT;
pub const DOG_HEIGHT: i16 = 89;

pub const JUMPING_FRAMES: u8 = 60;
pub const JUMP_SPEED: i16 = -25;

#[derive(Clone)]
pub struct DogContext {
    distance_max: i16,
    pub distance_min: i16,
    pub floor: i16,
    pub frame: u8,
    pub position: Point,
    pub velocity: Point,
}

impl DogContext {
    pub fn new(frame: u8, position: Point, velocity: Point) -> Self {
        DogContext {
            distance_max: 1000,
            distance_min: 300,
            floor: DOG_GROUND,
            frame,
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

    pub fn flee(mut self) -> Self {
        self.velocity.x = if self.position.x > 550 { -1 } else { 0 };
        log!("Dog starts fleeing {}", self.info());

        self
    }

    pub fn floor(&self) -> i16 {
        self.floor
    }

    pub fn reset_frame(mut self) -> Self {
        self.frame = 0;

        self
    }

    pub fn set_floor(mut self, bottom: i16) -> Self {
        self.floor = bottom - DOG_HEIGHT;
        log!(
            "DogContext: set floor to {}",
            if self.floor == DOG_GROUND {
                "Ground"
            } else {
                "Platform"
            }
        );

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

        if self.should_toggle_direction() {
            self.velocity.x *= -1;
            log!("Dog: toggled direction {}", self.info());
        }

        self
    }

    pub fn worry(mut self) -> Self {
        self.velocity.x = 4;
        self.distance_min = 50;
        log!("Dog worries {}", self.info());

        self
    }

    fn should_toggle_direction(&self) -> bool {
        let Point { x, y, .. } = self.position;
        let Point { x: vx, .. } = self.velocity;
        let on_floor = y == self.floor;

        let too_far = x > self.distance_max && vx >= 0;
        let too_close = x < self.distance_min && vx < 0;

        on_floor && (too_close || too_far)
    }
}
