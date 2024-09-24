use super::{
    jumping_worried_return::JumpingWorriedReturn, running_worried::RunningWorried, DogState,
};
use crate::game::dog::{
    context::{JUMP_SPEED, RUNNING_FRAMES},
    state_machine::DogStateMachine,
};

#[derive(Clone)]
pub struct ReturningWorried;

impl DogState<ReturningWorried> {
    pub fn jump(mut self) -> DogState<JumpingWorriedReturn> {
        log!("Dog ReturningWorried->JumpingWorriedReturn");
        self.context.velocity.y = JUMP_SPEED;

        DogState {
            context: self.context,
            _state: JumpingWorriedReturn,
        }
    }

    pub fn update(mut self) -> ReturningEndState {
        self.context = self.context.update(RUNNING_FRAMES);

        if self.context.position.x < 50 {
            ReturningEndState::Running(self.run_away())
        } else {
            ReturningEndState::Returning(self)
        }
    }

    fn run_away(self) -> DogState<RunningWorried> {
        log!("Dog ReturningWorried->RunningWorried");

        DogState {
            context: self.context.toggle_direction().reset_frame(),
            _state: RunningWorried,
        }
    }
}

pub enum ReturningEndState {
    Returning(DogState<ReturningWorried>),
    Running(DogState<RunningWorried>),
}

impl From<ReturningEndState> for DogStateMachine {
    fn from(end_state: ReturningEndState) -> Self {
        match end_state {
            ReturningEndState::Returning(returning) => returning.into(),
            ReturningEndState::Running(running) => running.into(),
        }
    }
}
