use super::{fleeing::Fleeing, jumping::Jumping, DogState};
use crate::{
    engine::rect::Point,
    game::{
        dog::{
            context::{DogContext, DOG_FLOOR, JUMP_SPEED, RUNNING_FRAMES},
            state_machine::DogStateMachine,
            states::returning_to_flee::ReturningToFlee,
        },
        HEIGHT,
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

    pub fn drop_from_platform(self) -> DogState<Running> {
        log!("Dog drops from platform");

        DogState {
            context: self.context.set_floor(HEIGHT),
            _state: Running,
        }
    }

    pub fn flee(mut self) -> FleeingEndState {
        if self.context.position.x > 550 {
            self.context.velocity.x *= 2; // screen starts scrolling left

            FleeingEndState::ReturningToFlee(self.return_to_flee())
        } else {
            log!("Dog Running->Fleeing {:?}", self.context.info());
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

    pub fn update(mut self) -> DogState<Running> {
        self.context = self.context.update(RUNNING_FRAMES);

        self
    }

    fn return_to_flee(self) -> DogState<ReturningToFlee> {
        log!("Dog Running->ReturningToFlee");

        DogState {
            context: self.context.toggle_direction(),
            _state: ReturningToFlee,
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
