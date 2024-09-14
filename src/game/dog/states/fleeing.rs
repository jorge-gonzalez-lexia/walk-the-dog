use super::{running_worried::RunningWorried, DogState};
use crate::game::dog::context::RUNNING_FRAMES;

#[derive(Clone)]
pub struct Fleeing;

impl DogState<Fleeing> {
    pub fn update(mut self) -> DogState<Fleeing> {
        self.context = self.context.update(RUNNING_FRAMES);

        self
    }

    pub fn worry(mut self) -> DogState<RunningWorried> {
        log!("Dog Fleeing->RunningWorried");

        self.context.velocity.x = 4;

        DogState {
            context: self.context,
            _state: RunningWorried,
        }
    }
}
