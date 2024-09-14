use crate::game::dog::context::RUNNING_FRAMES;

use super::{running::Running, DogState};

#[derive(Clone)]
pub struct Fleeing;

impl DogState<Fleeing> {
    pub fn update(mut self) -> DogState<Fleeing> {
        self.context = self.context.update(RUNNING_FRAMES);

        self
    }

    pub fn worry(mut self) -> DogState<Running> {
        self.context.velocity.x = 4;

        DogState {
            context: self.context,
            _state: Running,
        }
    }
}
