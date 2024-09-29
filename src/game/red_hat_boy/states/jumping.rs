use super::{falling::Falling, running::Running, RedHatBoyState};
use crate::game::{
    self,
    red_hat_boy::{context::FLOOR, state_machine::RedHatBoyStateMachine},
};

const JUMP_FRAME_NAME: &str = "Jump";
const JUMPING_FRAMES: u8 = 35;

#[derive(Clone, Copy)]
pub struct Jumping;

impl RedHatBoyState<Jumping> {
    pub fn frame_name(&self) -> &str {
        JUMP_FRAME_NAME
    }

    pub fn knock_out(self) -> RedHatBoyState<Falling> {
        log!("Jumping->Falling");

        RedHatBoyState {
            context: self.context.reset_frame().stop(),
            _state: Falling {},
        }
    }

    pub fn land_on(self, position: i16) -> RedHatBoyState<Running> {
        log!("Jumping->Running");

        RedHatBoyState {
            context: self.context.reset_frame().set_on(position),
            _state: Running,
        }
    }

    pub fn update(mut self) -> JumpingEndState {
        self.context = self.context.update(JUMPING_FRAMES);

        if self.context.position.y >= FLOOR {
            JumpingEndState::Landing(self.land_on(game::HEIGHT))
        } else {
            JumpingEndState::Jumping(self)
        }
    }
}

pub enum JumpingEndState {
    Jumping(RedHatBoyState<Jumping>),
    Landing(RedHatBoyState<Running>),
}

impl From<JumpingEndState> for RedHatBoyStateMachine {
    fn from(end_state: JumpingEndState) -> Self {
        match end_state {
            JumpingEndState::Landing(running_state) => running_state.into(),
            JumpingEndState::Jumping(jumping_state) => jumping_state.into(),
        }
    }
}
