use super::{running::Running, DogState};
use crate::game::dog::{context::JUMPING_FRAMES, state_machine::DogStateMachine};

#[derive(Clone)]
pub struct Jumping;

impl DogState<Jumping> {
    pub fn land_on(self, platform: i16) -> JumpingEndState {
        self.land(Some(platform))
    }

    pub fn frame_name(&self) -> String {
        let animation_frame = self.context().frame / 3;
        format!("l_{animation_frame:03}.png")
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
        log!(
            "Dog Jumping->Running (lands{})",
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

        JumpingEndState::Lands(DogState {
            context,
            _state: Running,
        })
    }
}

pub enum JumpingEndState {
    Jumping(DogState<Jumping>),
    Lands(DogState<Running>),
}

impl From<JumpingEndState> for DogStateMachine {
    fn from(end_state: JumpingEndState) -> Self {
        match end_state {
            JumpingEndState::Jumping(jumping) => jumping.into(),
            JumpingEndState::Lands(s) => s.into(),
        }
    }
}
