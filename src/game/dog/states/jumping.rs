use super::{running::Running, DogState};
use crate::game::dog::context::JUMPING_FRAMES;

#[derive(Clone)]
pub struct Jumping;

impl DogState<Jumping> {
    pub fn land_on(self, platform: i16) -> DogState<Running> {
        self.land(Some(platform))
    }

    pub fn frame_name(&self) -> String {
        let animation_frame = self.context().frame / 3;
        format!("l_{animation_frame:03}.png")
    }

    pub fn update(mut self) -> DogState<Jumping> {
        self.context = self.context.update(JUMPING_FRAMES);

        self
    }

    pub fn land(self, platform: Option<i16>) -> DogState<Running> {
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

        DogState {
            context,
            _state: Running,
        }
    }
}
