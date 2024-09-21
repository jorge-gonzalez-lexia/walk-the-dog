use super::{jumping_worried::JumpingWorried, returning_worried::ReturningWorried, DogState};
use crate::game::dog::{
    context::{JUMP_SPEED, RUNNING_FRAMES},
    state_machine::DogStateMachine,
};

#[derive(Clone)]
pub struct RunningWorried;

impl DogState<RunningWorried> {
    pub fn jump(mut self) -> DogState<JumpingWorried> {
        log!("Dog RunningWorried->JumpingWorried");
        self.context.velocity.y = JUMP_SPEED;

        DogState {
            context: self.context,
            _state: JumpingWorried,
        }
    }

    pub fn land_on(self, position: i16) -> DogState<RunningWorried> {
        DogState {
            context: self.context.set_on(position),
            _state: RunningWorried,
        }
    }
    pub fn update(mut self) -> RunningEndState {
        self.context = self.context.update(RUNNING_FRAMES);

        if self.context.position.x > 1000 {
            RunningEndState::Returning(self.return_to_boy())
        } else {
            RunningEndState::Running(self)
        }
    }

    fn return_to_boy(self) -> DogState<ReturningWorried> {
        log!(
            "Dog RunningWorried->ReturningWorried (velocity {})",
            self.context.velocity.x
        );

        DogState {
            context: self.context.toggle_direction(),
            _state: ReturningWorried,
        }
    }
}

pub enum RunningEndState {
    Returning(DogState<ReturningWorried>),
    Running(DogState<RunningWorried>),
}

impl From<RunningEndState> for DogStateMachine {
    fn from(end_state: RunningEndState) -> Self {
        match end_state {
            RunningEndState::Returning(returning) => returning.into(),
            RunningEndState::Running(running) => running.into(),
        }
    }
}
