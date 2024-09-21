use super::fleeing::Fleeing;
use crate::game::{
    self,
    dog::{
        context::{DOG_FLOOR, JUMPING_FRAMES},
        state_machine::DogStateMachine,
        states::DogState,
    },
};

#[derive(Clone)]
pub struct JumpingFlee;

impl DogState<JumpingFlee> {
    pub fn land_on(self, position: i16) -> DogState<Fleeing> {
        log!("Dog JumpingFlee->Fleeing (lands)");

        DogState {
            context: self.context.reset_frame().set_on(position),
            _state: Fleeing,
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
