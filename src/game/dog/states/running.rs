use super::{fleeing::Fleeing, returning::Returning, DogState};
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

    pub fn flee(mut self) -> FleeingEndState {
        if self.context.position.x > 550 {
            self.context.velocity.x *= 2; // screen starts scrolling left

            FleeingEndState::Returning(self.return_then_flee())
        } else {
            log!("Dog Running->Fleeing");
            self.context.velocity.x = 0; // screen starts scrolling left

            FleeingEndState::Fleeing(DogState {
                context: self.context,
                _state: Fleeing,
            })
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

    fn return_then_flee(self) -> DogState<Returning> {
        log!("Dog Running->Returning::then_flee");

        DogState {
            context: self.context.toggle_direction(),
            _state: Returning { then_flee: true },
        }
    }

    fn return_to_boy(self) -> DogState<Returning> {
        log!("Dog Running->Returning");

        DogState {
            context: self.context.toggle_direction(),
            _state: Returning { then_flee: false },
        }
    }
}

pub enum FleeingEndState {
    Fleeing(DogState<Fleeing>),
    Returning(DogState<Returning>),
}

impl From<FleeingEndState> for DogStateMachine {
    fn from(end_state: FleeingEndState) -> Self {
        match end_state {
            FleeingEndState::Fleeing(fleeing) => fleeing.into(),
            FleeingEndState::Returning(running) => running.into(),
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
