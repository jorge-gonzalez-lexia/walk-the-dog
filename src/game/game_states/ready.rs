use super::{walking::Walking, WalkTheDogState, WalkTheDogStateMachine};
use crate::{engine::input::KeyState, game::walk::Walk};

pub struct Ready;

impl WalkTheDogState<Ready> {
    pub fn new(walk: Walk) -> Self {
        WalkTheDogState {
            walk,
            _state: Ready,
        }
    }

    pub fn update(mut self, keystate: &KeyState) -> ReadyEndState {
        self.walk.boy.update();
        self.walk.dog.update();

        if keystate.is_pressed("ArrowRight") {
            ReadyEndState::Complete(self.start_running())
        } else {
            ReadyEndState::Continue(self)
        }
    }

    fn run_right(&mut self) {
        self.walk.boy.run_right();
    }

    fn start_running(mut self) -> WalkTheDogState<Walking> {
        self.run_right();

        WalkTheDogState {
            walk: self.walk,
            _state: Walking,
        }
    }
}

pub enum ReadyEndState {
    Complete(WalkTheDogState<Walking>),
    Continue(WalkTheDogState<Ready>),
}

impl From<ReadyEndState> for WalkTheDogStateMachine {
    fn from(state: ReadyEndState) -> Self {
        match state {
            ReadyEndState::Complete(walking) => walking.into(),
            ReadyEndState::Continue(ready) => ready.into(),
        }
    }
}
