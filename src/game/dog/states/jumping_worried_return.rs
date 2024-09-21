use super::{returning_worried::ReturningWorried, DogState};
use crate::game::{
    self,
    dog::{
        context::{DOG_FLOOR, JUMPING_FRAMES},
        state_machine::DogStateMachine,
    },
};

#[derive(Clone)]
pub struct JumpingWorriedReturn;

impl DogState<JumpingWorriedReturn> {
    pub fn land_on(self, position: i16) -> DogState<ReturningWorried> {
        log!("Dog JumpingWorriedReturn->ReturningWorried");

        DogState {
            context: self.context.reset_frame().set_on(position),
            _state: ReturningWorried,
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
