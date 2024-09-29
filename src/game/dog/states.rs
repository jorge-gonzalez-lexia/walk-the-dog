pub mod jumping;
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

    pub fn flee(mut self) -> DogState<S> {
        self.context = self.context.flee();

        self
    }

    pub fn worry(mut self) -> DogState<S> {
        self.context = self.context.worry();

        self
    }
}
