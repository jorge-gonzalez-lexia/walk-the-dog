use crate::{
    engine::rect::Point,
    game::{self, event_queue::GameEvent},
};

pub const RUNNING_FRAMES: u8 = 60;
pub const DOG_GROUND: i16 = game::HEIGHT - DOG_HEIGHT;
pub const DOG_HEIGHT: i16 = 81;

pub const JUMPING_FRAMES: u8 = 17 * 3;
pub const JUMP_SPEED: i16 = -25;

#[derive(Clone)]
pub struct DogContext {
    distance_max: i16,
    pub distance_min: i16,
    pub event_publisher: game::event_queue::EventPublisher,
    pub floor: i16,
    pub frame: u8,
    pub position: Point,
    pub velocity: Point,
}

impl DogContext {
    pub fn new(event_publisher: game::event_queue::EventPublisher) -> Self {
        let floor = DOG_GROUND;
        let position = Point { x: 10, y: floor };
        let velocity = Point { x: 4, y: 0 };

        DogContext {
            distance_max: 1000,
            distance_min: 300,
            event_publisher,
            floor,
            frame: 0,
            position,
            velocity,
        }
    }

    pub fn flee(mut self) -> Self {
        self.velocity.x = if self.position.x > 550 { -1 } else { 0 };
        log!("Dog starts fleeing {}", self.info());

        self
    }

    pub fn info(&self) -> String {
        format!(
            "pos={:?} v={:?} floor={:?}",
            self.position, self.velocity, self.floor
        )
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

    pub fn turn_around(mut self) -> Self {
        self.velocity.x *= -1;
        log!("Dog: turned around {}", self.info());

        self
    }

    pub fn update(mut self, frame_count: u8) -> Self {
        if self.frame < frame_count {
            self.frame += 1;
        } else {
            self.frame = 0
        }

        // vertical movement
        let was_on_floor = self.on_floor();
        if self.velocity.y < game::TERMINAL_VELOCITY {
            self.velocity.y += game::GRAVITY;
        }

        self.position.y += self.velocity.y;

        if self.position.y > self.floor {
            self.position.y = self.floor;
        }

        if self.on_floor() && !was_on_floor {
            self.event_publisher.publish(GameEvent::DogLanded);
        }

        // horizontal movement
        self.position.x += self.velocity.x;
        if self.too_close() {
            self.event_publisher.publish(GameEvent::DogTooClose);
        } else if self.too_far() {
            self.event_publisher.publish(GameEvent::DogTooFar);
        }

        self
    }

    pub fn worry(mut self) -> Self {
        self.velocity.x = 4;
        self.distance_min = 50;
        log!("Dog worries {}", self.info());

        self
    }

    fn on_floor(&self) -> bool {
        self.position.y == self.floor
    }

    fn too_close(&self) -> bool {
        self.on_floor() && self.position.x < self.distance_min && self.velocity.x < 0
    }

    fn too_far(&self) -> bool {
        self.on_floor() && self.position.x > self.distance_max && self.velocity.x >= 0
    }
}
