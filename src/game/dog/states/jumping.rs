use super::{running::Running, DogState};
use crate::game::dog::{context::JUMPING_FRAMES, state_machine::DogStateMachine};

#[derive(Clone)]
pub struct Jumping;

impl DogState<Jumping> {
    pub fn land(self) -> DogState<Running> {
        log!("Dog Jumping->Running (lands)");

        DogState {
            context: self.context.reset_frame(),
            _state: Running,
        }
    }

    pub fn update(mut self) -> JumpingEndState {
        self.context = self.context.update(JUMPING_FRAMES);

        let floor = self.context.floor();
        if self.context.velocity.y > 0 && self.context.position.y == floor {
            JumpingEndState::Landing(self.land())
        } else {
            JumpingEndState::Jumping(self)
        }
    }
}

pub enum JumpingEndState {
    Jumping(DogState<Jumping>),
    Landing(DogState<Running>),
}

impl From<JumpingEndState> for DogStateMachine {
    fn from(end_state: JumpingEndState) -> Self {
        match end_state {
            JumpingEndState::Jumping(jumping) => jumping.into(),
            JumpingEndState::Landing(running) => running.into(),
        }
    }
}
