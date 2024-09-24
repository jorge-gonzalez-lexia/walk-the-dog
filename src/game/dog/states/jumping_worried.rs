use super::{running_worried::RunningWorried, DogState};
use crate::game::dog::{context::JUMPING_FRAMES, state_machine::DogStateMachine};

#[derive(Clone)]
pub struct JumpingWorried;

impl DogState<JumpingWorried> {
    pub fn land_on(self, platform: i16) -> DogState<RunningWorried> {
        log!("Dog JumpingWorried->RunningWorried (lands)");

        DogState {
            context: self.context.reset_frame().set_floor(platform),
            _state: RunningWorried,
        }
    }

    pub fn update(mut self) -> JumpingEndState {
        self.context = self.context.update(JUMPING_FRAMES);

        if self.context.velocity.y > 0 && self.context.position.y == self.context.floor() {
            JumpingEndState::Landing(self.land())
        } else {
            JumpingEndState::Jumping(self)
        }
    }

    fn land(self) -> DogState<RunningWorried> {
        log!("Dog JumpingWorried->RunningWorried (lands)");

        DogState {
            context: self.context.reset_frame(),
            _state: RunningWorried,
        }
    }
}

pub enum JumpingEndState {
    Jumping(DogState<JumpingWorried>),
    Landing(DogState<RunningWorried>),
}

impl From<JumpingEndState> for DogStateMachine {
    fn from(end_state: JumpingEndState) -> Self {
        match end_state {
            JumpingEndState::Jumping(jumping) => jumping.into(),
            JumpingEndState::Landing(running) => running.into(),
        }
    }
}
