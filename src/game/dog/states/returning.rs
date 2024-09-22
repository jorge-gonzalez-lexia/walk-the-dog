use super::{fleeing::Fleeing, jumping_return::JumpingReturn, running::Running, DogState};
use crate::{
    engine::rect::Rect,
    game::dog::{
        context::{JUMP_SPEED, RUNNING_FRAMES},
        state_machine::DogStateMachine,
    },
};

#[derive(Clone)]
pub struct Returning;

impl DogState<Returning> {
    pub fn flee(mut self) -> DogState<Fleeing> {
        log!("Dog Returning->Fleeing {}", self.context.position.x);
        self.context.velocity.x = 0;

        DogState {
            context: self.context,
            _state: Fleeing,
        }
    }

    pub fn jump(mut self) -> DogState<JumpingReturn> {
        log!("Dog Returning->JumpingReturn");
        self.context.velocity.y = JUMP_SPEED;

        DogState {
            context: self.context,
            _state: JumpingReturn,
        }
    }

    pub fn jump_to(self, platform: Rect) -> DogState<JumpingReturn> {
        log!("Dog Returning->JumpingTo {platform:?}");

        DogState {
            context: self.context.jump_to(platform),
            _state: JumpingReturn,
        }
    }

    pub fn land_on(self, position: i16) -> DogState<Returning> {
        DogState {
            context: self.context.set_on(position),
            _state: Returning,
        }
    }

    pub fn update(mut self) -> ReturningEndState {
        self.context = self.context.update(RUNNING_FRAMES);

        if self.context.position.x < 300 {
            ReturningEndState::Running(self.run_away())
        } else {
            ReturningEndState::Returning(self)
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
    Returning(DogState<Returning>),
    Running(DogState<Running>),
}

impl From<ReturningEndState> for DogStateMachine {
    fn from(end_state: ReturningEndState) -> Self {
        match end_state {
            ReturningEndState::Returning(returning) => returning.into(),
            ReturningEndState::Running(running) => running.into(),
        }
    }
}
