use super::{jumping_flee::JumpingFlee, returning_worried::ReturningWorried, DogState};
use crate::game::dog::context::{JUMP_SPEED, RUNNING_FRAMES};

#[derive(Clone)]
pub struct Fleeing;

impl DogState<Fleeing> {
    pub fn jump(mut self) -> DogState<JumpingFlee> {
        log!("Dog Fleeing->JumpingFlee");
        self.context.velocity.y = JUMP_SPEED;

        DogState {
            context: self.context,
            _state: JumpingFlee,
        }
    }

    pub fn land_on(self, position: i16) -> DogState<Fleeing> {
        DogState {
            context: self.context.set_on(position),
            _state: Fleeing,
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
