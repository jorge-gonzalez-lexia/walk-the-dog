use super::fleeing::Fleeing;
use crate::game::dog::{context::JUMPING_FRAMES, state_machine::DogStateMachine, states::DogState};

#[derive(Clone)]
pub struct JumpingFlee;

impl DogState<JumpingFlee> {
    pub fn land_on(self, platform: i16) -> DogState<Fleeing> {
        log!("Dog JumpingFlee->Fleeing (lands on platform)");

        DogState {
            context: self.context.reset_frame().set_floor(platform),
            _state: Fleeing,
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

    fn land(self) -> DogState<Fleeing> {
        log!("Dog JumpingFlee->Fleeing (lands)");

        DogState {
            context: self.context.reset_frame(),
            _state: Fleeing,
        }
    }
}

pub enum JumpingEndState {
    Jumping(DogState<JumpingFlee>),
    Landing(DogState<Fleeing>),
}

impl From<JumpingEndState> for DogStateMachine {
    fn from(end_state: JumpingEndState) -> Self {
        match end_state {
            JumpingEndState::Jumping(jumping) => jumping.into(),
            JumpingEndState::Landing(fleeing) => fleeing.into(),
        }
    }
}
