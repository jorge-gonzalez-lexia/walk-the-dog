use super::{returning_to_flee::ReturningToFlee, DogState};
use crate::game::{
    self,
    dog::{
        context::{DOG_FLOOR, JUMPING_FRAMES},
        state_machine::DogStateMachine,
    },
};

#[derive(Clone)]
pub struct JumpingFleeReturn;

impl DogState<JumpingFleeReturn> {
    pub fn land_on(self, position: i16) -> DogState<ReturningToFlee> {
        log!("Dog JumpingFleeReturn->ReturningToFlee (lands)");

        DogState {
            context: self.context.reset_frame().set_on(position),
            _state: ReturningToFlee,
        }
    }

    pub fn update(mut self) -> JumpingEndState {
        self.context = self.context.update(JUMPING_FRAMES);

        if self.context.position.y >= DOG_FLOOR {
            JumpingEndState::Landing(self.land_on(game::HEIGHT))
        } else {
            JumpingEndState::Jumping(self)
        }
    }
}

pub enum JumpingEndState {
    Jumping(DogState<JumpingFleeReturn>),
    Landing(DogState<ReturningToFlee>),
}

impl From<JumpingEndState> for DogStateMachine {
    fn from(end_state: JumpingEndState) -> Self {
        match end_state {
            JumpingEndState::Jumping(jumping) => jumping.into(),
            JumpingEndState::Landing(returning) => returning.into(),
        }
    }
}
