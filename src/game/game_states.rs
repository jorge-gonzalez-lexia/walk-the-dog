mod game_over;
pub mod ready;
mod walking;

use super::walk::Walk;
use crate::engine::{input::KeyState, renderer::Renderer};
use game_over::GameOver;
use ready::Ready;
use walking::Walking;

pub enum WalkTheDogStateMachine {
    Ready(WalkTheDogState<Ready>),
    Walking(WalkTheDogState<Walking>),
    GameOver(WalkTheDogState<GameOver>),
}

impl WalkTheDogStateMachine {
    pub fn draw(&self, renderer: &Renderer) {
        match self {
            WalkTheDogStateMachine::GameOver(state) => state.draw(renderer),
            WalkTheDogStateMachine::Ready(state) => state.draw(renderer),
            WalkTheDogStateMachine::Walking(state) => state.draw(renderer),
        }
    }

    pub fn update(self, keystate: &KeyState) -> Self {
        match self {
            WalkTheDogStateMachine::GameOver(state) => state.update().into(),
            WalkTheDogStateMachine::Ready(state) => state.update(keystate).into(),
            WalkTheDogStateMachine::Walking(state) => state.update(keystate).into(),
        }
    }
}

impl From<WalkTheDogState<GameOver>> for WalkTheDogStateMachine {
    fn from(state: WalkTheDogState<GameOver>) -> Self {
        WalkTheDogStateMachine::GameOver(state)
    }
}

impl From<WalkTheDogState<Ready>> for WalkTheDogStateMachine {
    fn from(state: WalkTheDogState<Ready>) -> Self {
        WalkTheDogStateMachine::Ready(state)
    }
}

impl From<WalkTheDogState<Walking>> for WalkTheDogStateMachine {
    fn from(state: WalkTheDogState<Walking>) -> Self {
        WalkTheDogStateMachine::Walking(state)
    }
}

pub struct WalkTheDogState<T> {
    pub walk: Walk,
    pub _state: T,
}

impl<T> WalkTheDogState<T> {
    fn draw(&self, renderer: &Renderer) {
        self.walk.draw(renderer);
    }
}
