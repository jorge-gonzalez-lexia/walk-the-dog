use super::{returning_worried::ReturningWorried, DogState};
use crate::game::dog::{context::JUMPING_FRAMES, state_machine::DogStateMachine};

#[derive(Clone)]
pub struct JumpingWorriedReturn;

impl DogState<JumpingWorriedReturn> {
    pub fn land_on(self, platform: i16) -> DogState<ReturningWorried> {
        log!("Dog JumpingWorriedReturn->ReturningWorried (lands)");

        DogState {
            context: self.context.reset_frame().set_floor(platform),
            _state: ReturningWorried,
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

    fn land(self) -> DogState<ReturningWorried> {
        log!("Dog JumpingReturn->Returning (lands)");

        DogState {
            context: self.context.reset_frame(),
            _state: ReturningWorried,
        }
    }
}

pub enum JumpingEndState {
    Jumping(DogState<JumpingWorriedReturn>),
    Landing(DogState<ReturningWorried>),
}

impl From<JumpingEndState> for DogStateMachine {
    fn from(end_state: JumpingEndState) -> Self {
        match end_state {
            JumpingEndState::Jumping(jumping) => jumping.into(),
            JumpingEndState::Landing(returning) => returning.into(),
        }
    }
}
