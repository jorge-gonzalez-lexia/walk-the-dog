use super::{jumping_flee::JumpingFlee, returning_worried::ReturningWorried, DogState};
use crate::{
    engine::rect::Rect,
    game::{
        dog::context::{JUMP_SPEED, RUNNING_FRAMES},
        HEIGHT,
    },
};

#[derive(Clone)]
pub struct Fleeing;

impl DogState<Fleeing> {
    pub fn drop_from_platform(self) -> DogState<Fleeing> {
        log!("Dog drops from platform");

        DogState {
            context: self.context.set_floor(HEIGHT),
            _state: Fleeing,
        }
    }

    pub fn jump(mut self) -> DogState<JumpingFlee> {
        log!("Dog Fleeing->JumpingFlee");
        self.context.velocity.y = JUMP_SPEED;

        DogState {
            context: self.context,
            _state: JumpingFlee,
        }
    }

    pub fn update(mut self) -> DogState<Fleeing> {
        self.context = self.context.update(RUNNING_FRAMES);

        self
    }

    pub fn worry(mut self) -> DogState<ReturningWorried> {
        log!("Dog Fleeing->ReturningWorried");

        self.context.velocity.x = -4;

        DogState {
            context: self.context,
            _state: ReturningWorried,
        }
    }
}
