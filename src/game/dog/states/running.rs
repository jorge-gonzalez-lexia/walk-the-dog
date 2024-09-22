use super::{fleeing::Fleeing, jumping::Jumping, returning::Returning, DogState};
use crate::{
    engine::rect::{Point, Rect},
    game::dog::{
        context::{DogContext, DOG_FLOOR, JUMP_SPEED, RUNNING_FRAMES},
        state_machine::DogStateMachine,
        states::returning_to_flee::ReturningToFlee,
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
                    y: DOG_FLOOR,
                },
                Point { x: 4, y: 0 },
            ),
            _state: Running,
        }
    }

    pub fn flee(mut self) -> FleeingEndState {
        if self.context.position.x > 550 {
            self.context.velocity.x *= 2; // screen starts scrolling left

            FleeingEndState::ReturningToFlee(self.return_to_flee())
        } else {
            log!("Dog Running->Fleeing");
            self.context.velocity.x = 0; // screen starts scrolling left

            FleeingEndState::Fleeing(DogState {
                context: self.context,
                _state: Fleeing,
            })
        }
    }

    pub fn jump(mut self) -> DogState<Jumping> {
        log!("Dog Running->Jumping");
        self.context.velocity.y = JUMP_SPEED;

        DogState {
            context: self.context,
            _state: Jumping,
        }
    }

    pub fn jump_to(self, platform: Rect) -> DogState<Jumping> {
        log!("Dog Running->JumpingTo {platform:?}");

        DogState {
            context: self.context.jump_to(platform),
            _state: Jumping,
        }
    }

    pub fn land_on(self, position: i16) -> DogState<Running> {
        DogState {
            context: self.context.set_on(position),
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

    fn return_to_flee(self) -> DogState<ReturningToFlee> {
        log!("Dog Running->ReturningToFlee");

        DogState {
            context: self.context.toggle_direction(),
            _state: ReturningToFlee,
        }
    }

    fn return_to_boy(self) -> DogState<Returning> {
        log!("Dog Running->Returning {}", self.context.position.x);

        DogState {
            context: self.context.toggle_direction(),
            _state: Returning,
        }
    }
}

pub enum FleeingEndState {
    Fleeing(DogState<Fleeing>),
    ReturningToFlee(DogState<ReturningToFlee>),
}

impl From<FleeingEndState> for DogStateMachine {
    fn from(end_state: FleeingEndState) -> Self {
        match end_state {
            FleeingEndState::Fleeing(fleeing) => fleeing.into(),
            FleeingEndState::ReturningToFlee(returning) => returning.into(),
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
