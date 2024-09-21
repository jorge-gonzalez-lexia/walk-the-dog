use super::{running_worried::RunningWorried, DogState};
use crate::game::{
    self,
    dog::{
        context::{DOG_FLOOR, JUMPING_FRAMES},
        state_machine::DogStateMachine,
    },
};

#[derive(Clone)]
pub struct JumpingWorried;

impl DogState<JumpingWorried> {
    pub fn land_on(self, position: i16) -> DogState<RunningWorried> {
        log!("Dog JumpingWorried->RunningWorried (lands)");

        DogState {
            context: self.context.reset_frame().set_on(position),
            _state: RunningWorried,
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
