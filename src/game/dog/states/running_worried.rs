use super::{jumping_worried::JumpingWorried, returning_worried::ReturningWorried, DogState};
use crate::game::{
    dog::{
        context::{JUMP_SPEED, RUNNING_FRAMES},
        state_machine::DogStateMachine,
    },
    HEIGHT,
};

#[derive(Clone)]
pub struct RunningWorried;

impl DogState<RunningWorried> {
    pub fn drop_from_platform(self) -> DogState<RunningWorried> {
        log!("Dog drops from platform");

        DogState {
            context: self.context.set_floor(HEIGHT),
            _state: RunningWorried,
        }
    }

    pub fn jump(mut self) -> DogState<JumpingWorried> {
        log!("Dog RunningWorried->JumpingWorried");
        self.context.velocity.y = JUMP_SPEED;

        DogState {
            context: self.context,
            _state: JumpingWorried,
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
