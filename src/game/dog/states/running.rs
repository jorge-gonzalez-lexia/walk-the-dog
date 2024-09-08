use crate::{
    engine::rect::Point,
    game::{self, dog::context::DogContext},
};

use super::DogState;

pub struct Running;

impl DogState<Running> {
    pub fn new() -> Self {
        DogState {
            context: DogContext::new(Point {
                x: 10,
                y: game::FLOOR + 27,
            }),
            _state: Running,
        }
    }
}
