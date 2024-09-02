use super::{ready::Ready, WalkTheDogState, WalkTheDogStateMachine};
use crate::{browser, game::walk::Walk};
use futures::channel::mpsc::UnboundedReceiver;

pub struct GameOver {
    pub new_game_event: UnboundedReceiver<()>,
}

impl GameOver {
    pub fn new_game_pressed(&mut self) -> bool {
        matches!(self.new_game_event.try_next(), Ok(Some(())))
    }
}

impl WalkTheDogState<GameOver> {
    pub fn update(mut self) -> GameOverEndState {
        if self._state.new_game_pressed() {
            GameOverEndState::Complete(self.new_game())
        } else {
            GameOverEndState::Continue(self)
        }
    }

    fn new_game(self) -> WalkTheDogState<Ready> {
        browser::hide_ui();

        WalkTheDogState {
            walk: Walk::reset(self.walk),
            _state: Ready,
        }
    }
}

pub enum GameOverEndState {
    Complete(WalkTheDogState<Ready>),
    Continue(WalkTheDogState<GameOver>),
}

impl From<GameOverEndState> for WalkTheDogStateMachine {
    fn from(state: GameOverEndState) -> Self {
        match state {
            GameOverEndState::Complete(ready) => ready.into(),
            GameOverEndState::Continue(game_over) => game_over.into(),
        }
    }
}
