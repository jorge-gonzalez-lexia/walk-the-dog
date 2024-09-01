use super::{falling::Falling, running::Running, RedHatBoyState};
use crate::game::red_hat_boy::state_machine::RedHatBoyStateMachine;

const SLIDE_FRAME_NAME: &str = "Slide";
const SLIDING_FRAMES: u8 = 14;

#[derive(Clone, Copy)]
pub struct Sliding;

impl RedHatBoyState<Sliding> {
    pub fn frame_name(&self) -> &str {
        SLIDE_FRAME_NAME
    }

    pub fn knock_out(self) -> RedHatBoyState<Falling> {
        log!("Sliding->Falling");

        RedHatBoyState {
            context: self.context().clone().reset_frame().stop(),
            _state: Falling {},
        }
    }

    pub fn land_on(self, position: i16) -> RedHatBoyState<Sliding> {
        RedHatBoyState {
            context: self.context.set_on(position),
            _state: Sliding,
        }
    }

    pub fn update(mut self) -> SlidingEndState {
        self.context = self.context.update(SLIDING_FRAMES);

        if self.context.frame >= SLIDING_FRAMES {
            SlidingEndState::Complete(self.stand())
        } else {
            SlidingEndState::Sliding(self)
        }
    }

    fn stand(self) -> RedHatBoyState<Running> {
        log!("Sliding->Running");

        RedHatBoyState {
            context: self.context().clone().reset_frame(),
            _state: Running {},
        }
    }
}

pub enum SlidingEndState {
    Complete(RedHatBoyState<Running>),
    Sliding(RedHatBoyState<Sliding>),
}

impl From<SlidingEndState> for RedHatBoyStateMachine {
    fn from(end_state: SlidingEndState) -> Self {
        match end_state {
            SlidingEndState::Complete(running_state) => running_state.into(),
            SlidingEndState::Sliding(sliding_state) => sliding_state.into(),
        }
    }
}
