pub mod fleeing;
pub mod returning;
pub mod returning_to_flee;
pub mod running;

use super::context::DogContext;

#[derive(Clone)]
pub struct DogState<S> {
    context: DogContext,
    _state: S,
}

impl<S> DogState<S> {
    pub fn context(&self) -> &DogContext {
        &self.context
    }
}
