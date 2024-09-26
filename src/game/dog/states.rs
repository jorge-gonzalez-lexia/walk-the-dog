pub mod jumping;
pub mod jumping_worried;
pub mod jumping_worried_return;
pub mod returning_worried;
pub mod running;
pub mod running_worried;

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
