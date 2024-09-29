use super::{jumping::Jumping, DogState};
use crate::{
    engine::rect::Point,
    game::{
        dog::context::{DogContext, DOG_GROUND, JUMP_SPEED, RUNNING_FRAMES},
        HEIGHT,
    },
};

#[derive(Clone)]
pub struct Running;

impl DogState<Running> {
    pub fn new() -> Self {
        log!("->Dog::Running");

        DogState {
            context: DogContext::new(
                0,
                Point {
                    x: 10,
                    y: DOG_GROUND,
                },
                Point { x: 4, y: 0 },
            ),
            _state: Running,
        }
    }

    pub fn drop_from_platform(self) -> DogState<Running> {
        log!("Dog drops from platform");

        DogState {
            context: self.context.set_floor(HEIGHT),
            _state: Running,
        }
    }

    pub fn frame_name(&self) -> String {
        let animation_frame = self.context().frame / 3;
        format!("rr_{animation_frame:03}.png")
    }

    pub fn jump(mut self) -> DogState<Jumping> {
        log!("Dog Running->Jumping");
        self.context.velocity.y = JUMP_SPEED;

        DogState {
            context: self.context.reset_frame(),
            _state: Jumping,
        }
    }

    pub fn update(mut self) -> DogState<Running> {
        self.context = self.context.update(RUNNING_FRAMES);

        self
    }
}
