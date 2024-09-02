use super::WalkTheDogState;
use crate::engine::input::KeyState;

pub struct Walking;

impl WalkTheDogState<Walking> {
    pub fn update(self, keystate: &KeyState) -> WalkTheDogState<Walking> {
        self
    }
}
