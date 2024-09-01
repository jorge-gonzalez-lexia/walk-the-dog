use super::{knocked_out::KnockedOut, RedHatBoyState};
use crate::game::red_hat_boy::state_machine::RedHatBoyStateMachine;

const FALL_FRAME_NAME: &str = "Dead";
const FALLING_FRAMES: u8 = 29;

#[derive(Clone, Copy)]
pub struct Falling;

impl RedHatBoyState<Falling> {
    pub fn frame_name(&self) -> &str {
        FALL_FRAME_NAME
    }

    pub fn update(mut self) -> FallingEndState {
        self.context = self.context.update(FALLING_FRAMES);

        if self.context.frame >= FALLING_FRAMES {
            FallingEndState::Complete(self.dead())
        } else {
            FallingEndState::Falling(self)
        }
    }

    fn dead(self) -> RedHatBoyState<KnockedOut> {
        log!("Falling->KnockedOut");

        RedHatBoyState {
            context: self.context.play_ko_sfx(),
            _state: KnockedOut {},
        }
    }
}

pub enum FallingEndState {
    Complete(RedHatBoyState<KnockedOut>),
    Falling(RedHatBoyState<Falling>),
}

impl From<FallingEndState> for RedHatBoyStateMachine {
    fn from(end_state: FallingEndState) -> Self {
        match end_state {
            FallingEndState::Complete(knocked_out_state) => knocked_out_state.into(),
            FallingEndState::Falling(falling_state) => falling_state.into(),
        }
    }
}
