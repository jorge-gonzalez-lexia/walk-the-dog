use super::{fleeing::Fleeing, running::Running, DogState};
use crate::game::dog::{context::RUNNING_FRAMES, state_machine::DogStateMachine};

#[derive(Clone)]
pub struct Returning {
    pub then_flee: bool,
}

impl DogState<Returning> {
    pub fn update(mut self) -> ReturningEndState {
        self.context = self.context.update(RUNNING_FRAMES);

        if self.context.position.x < 300 {
            if self._state.then_flee {
                ReturningEndState::Fleeing(self.flee())
            } else {
                ReturningEndState::Running(self.run_away())
            }
        } else {
            ReturningEndState::Returning(self)
        }
    }

    pub fn flee(mut self) -> DogState<Fleeing> {
        log!("Dog Returning->Fleeing {}", self.context.position.x);
        self.context.velocity.x = 0;

        DogState {
            context: self.context,
            _state: Fleeing,
        }
    }

    fn run_away(self) -> DogState<Running> {
        log!("Dog Returning->Running {}", self.context.position.x);

        DogState {
            context: self.context.toggle_direction().reset_frame(),
            _state: Running,
        }
    }
}

pub enum ReturningEndState {
    Fleeing(DogState<Fleeing>),
    Returning(DogState<Returning>),
    Running(DogState<Running>),
}

impl From<ReturningEndState> for DogStateMachine {
    fn from(end_state: ReturningEndState) -> Self {
        match end_state {
            ReturningEndState::Fleeing(fleeing) => fleeing.into(),
            ReturningEndState::Returning(returning) => returning.into(),
            ReturningEndState::Running(running) => running.into(),
        }
    }
}
