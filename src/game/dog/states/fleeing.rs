use crate::game::dog::context::RUNNING_FRAMES;

use super::DogState;

#[derive(Clone)]
pub struct Fleeing;

impl DogState<Fleeing> {
    pub fn update(mut self) -> DogState<Fleeing> {
        self.context = self.context.update(RUNNING_FRAMES);

        self
    }
}
