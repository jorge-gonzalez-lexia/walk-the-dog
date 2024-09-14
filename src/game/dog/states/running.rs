use super::{returning::Returning, DogState};
use crate::{
    engine::rect::Point,
    game::{
        self,
        dog::{
            context::{DogContext, RUNNING_FRAMES},
            state_machine::DogStateMachine,
        },
    },
};

#[derive(Clone)]
pub struct Running;

impl DogState<Running> {
    pub fn new() -> Self {
        log!("->Dog::Running");

        DogState {
            context: DogContext::new(
                0,
                Point {
                    x: 10,
                    y: game::FLOOR + 27,
                },
                Point { x: 4, y: 0 },
            ),
            _state: Running,
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

    fn return_to_boy(self) -> DogState<Returning> {
        log!("Dog Running->Returning");

        DogState {
            context: self.context.toggle_direction().reset_frame(),
            _state: Returning,
        }
    }
}

pub enum RunningEndState {
    Returning(DogState<Returning>),
    Running(DogState<Running>),
}

impl From<RunningEndState> for DogStateMachine {
    fn from(end_state: RunningEndState) -> Self {
        match end_state {
            RunningEndState::Returning(returning) => returning.into(),
            RunningEndState::Running(running) => running.into(),
        }
    }
}
