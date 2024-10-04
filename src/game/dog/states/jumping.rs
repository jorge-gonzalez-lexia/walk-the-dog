use super::{running::Running, DogState};
use crate::game::dog::context::JUMPING_FRAMES;

#[derive(Clone)]
pub struct Jumping;

impl DogState<Jumping> {
    pub fn frame_name(&self) -> String {
        let animation_frame = self.context().frame / 3;
        format!("l_{animation_frame:03}.png")
    }

    pub fn land_on(self, platform: i16) -> DogState<Running> {
        log!(
            "Dog: Jumping->Running (landed on platform) {}",
            self.context.info()
        );

        DogState {
            context: self.context.set_floor(platform).reset_frame(),
            _state: Running,
        }
    }

    pub fn land_on_ground(self) -> DogState<Running> {
        log!(
            "Dog: Jumping->Running (landed on ground) {}",
            self.context.info()
        );

        DogState {
            context: self.context.reset_frame(),
            _state: Running,
        }
    }

    pub fn update(mut self) -> DogState<Jumping> {
        self.context = self.context.update(JUMPING_FRAMES);

        self
    }
}
