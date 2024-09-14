use super::{fleeing::Fleeing, DogState};
use crate::game::dog::{context::RUNNING_FRAMES, state_machine::DogStateMachine};

#[derive(Clone)]
pub struct ReturningToFlee;

impl DogState<ReturningToFlee> {
    pub fn update(mut self) -> ReturningEndState {
        self.context = self.context.update(RUNNING_FRAMES);

        if self.context.position.x < 300 {
            ReturningEndState::Fleeing(self.flee())
        } else {
            ReturningEndState::Returning(self)
        }
    }

    pub fn flee(mut self) -> DogState<Fleeing> {
        log!("Dog ReturningToFlee->Fleeing {}", self.context.position.x);
        self.context.velocity.x = 0;

        DogState {
            context: self.context,
            _state: Fleeing,
        }
    }
}

pub enum ReturningEndState {
    Returning(DogState<ReturningToFlee>),
    Fleeing(DogState<Fleeing>),
}

impl From<ReturningEndState> for DogStateMachine {
    fn from(end_state: ReturningEndState) -> Self {
        match end_state {
            ReturningEndState::Returning(returning) => returning.into(),
            ReturningEndState::Fleeing(running) => running.into(),
        }
    }
}
