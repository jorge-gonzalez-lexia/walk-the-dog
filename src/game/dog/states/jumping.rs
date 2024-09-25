use super::{running::Running, DogState};
use crate::game::dog::{
    context::JUMPING_FRAMES, state_machine::DogStateMachine, states::returning::Returning,
};

#[derive(Clone)]
pub struct Jumping;

impl DogState<Jumping> {
    pub fn land_on(self, platform: i16) -> JumpingEndState {
        self.land(Some(platform))
    }

    pub fn update(mut self) -> JumpingEndState {
        self.context = self.context.update(JUMPING_FRAMES);

        if self.context.velocity.y > 0 && self.context.position.y == self.context.floor() {
            self.land(None)
        } else {
            JumpingEndState::Jumping(self)
        }
    }

    fn land(self, platform: Option<i16>) -> JumpingEndState {
        let is_returning = self.context.velocity.x < 0;
        log!(
            "Dog Jumping->{} (lands{})",
            if is_returning { "Returning" } else { "Running" },
            if platform.is_some() {
                " on platform"
            } else {
                ""
            }
        );

        let context = if let Some(platform) = platform {
            self.context.set_floor(platform).reset_frame()
        } else {
            self.context.reset_frame()
        };
        if is_returning {
            JumpingEndState::LandAndReturn(DogState {
                context,
                _state: Returning,
            })
        } else {
            JumpingEndState::LandAndRun(DogState {
                context,
                _state: Running,
            })
        }
    }
}

pub enum JumpingEndState {
    Jumping(DogState<Jumping>),
    LandAndReturn(DogState<Returning>),
    LandAndRun(DogState<Running>),
}

impl From<JumpingEndState> for DogStateMachine {
    fn from(end_state: JumpingEndState) -> Self {
        match end_state {
            JumpingEndState::Jumping(jumping) => jumping.into(),
            JumpingEndState::LandAndReturn(s) => s.into(),
            JumpingEndState::LandAndRun(s) => s.into(),
        }
    }
}
