use super::{fleeing::Fleeing, returning_worried::ReturningWorried, DogState};
use crate::game::dog::{
    context::{JUMP_SPEED, RUNNING_FRAMES},
    state_machine::DogStateMachine,
    states::jumping_flee_return::JumpingFleeReturn,
};

#[derive(Clone)]
pub struct ReturningToFlee;

impl DogState<ReturningToFlee> {
    pub fn jump(mut self) -> DogState<JumpingFleeReturn> {
        log!("Dog ReturningToFlee->JumpingFleeReturn");
        self.context.velocity.y = JUMP_SPEED;

        DogState {
            context: self.context,
            _state: JumpingFleeReturn,
        }
    }

    pub fn land_on(self, platform: i16) -> DogState<ReturningToFlee> {
        DogState {
            context: self.context.set_floor(platform),
            _state: ReturningToFlee,
        }
    }

    pub fn update(mut self) -> ReturningEndState {
        self.context = self.context.update(RUNNING_FRAMES);

        if self.context.position.x < 300 {
            ReturningEndState::Fleeing(self.flee())
        } else {
            ReturningEndState::Returning(self)
        }
    }

    pub fn worry(self) -> DogState<ReturningWorried> {
        log!("Dog ReturningToFlee->ReturningWorried");

        DogState {
            context: self.context,
            _state: ReturningWorried,
        }
    }

    fn flee(mut self) -> DogState<Fleeing> {
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
