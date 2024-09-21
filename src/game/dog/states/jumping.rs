use crate::game::{
    self,
    dog::{context::DOG_FLOOR, state_machine::DogStateMachine},
};

use super::{running::Running, DogState};

#[derive(Clone)]
pub struct Jumping;

impl DogState<Jumping> {
    pub fn land_on(self, position: i16) -> DogState<Running> {
        log!("Dog Jumping->Running (lands)");

        DogState {
            context: self.context.reset_frame().set_on(position),
            _state: Running,
        }
    }

    pub fn update(mut self) -> JumpingEndState {
        self.context = self.context.update(5); // TODO jumping frames

        if self.context.position.y >= DOG_FLOOR {
            JumpingEndState::Landing(self.land_on(game::HEIGHT))
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
