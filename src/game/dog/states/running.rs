use crate::{
    engine::rect::Point,
    game::{self, dog::context::DogContext},
};

use super::DogState;

#[derive(Clone)]
pub struct Running;

impl DogState<Running> {
    pub fn new() -> Self {
        DogState {
            context: DogContext::new(
                0,
                Point {
                    x: 10,
                    y: game::FLOOR + 27,
                },
            ),
            _state: Running,
        }
    }

    pub fn update(mut self) -> Self {
        self.context = self.context.update(5);

        self
    }
}
