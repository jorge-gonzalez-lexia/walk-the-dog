use super::{falling::Falling, jumping::Jumping, sliding::Sliding, RedHatBoyState};

const RUN_FRAME_NAME: &str = "Run";
const RUNNING_FRAMES: u8 = 23;

#[derive(Clone, Copy)]
pub struct Running;

const JUMP_SPEED: i16 = -25;

impl RedHatBoyState<Running> {
    pub fn frame_name(&self) -> &str {
        RUN_FRAME_NAME
    }

    pub fn jump(self) -> RedHatBoyState<Jumping> {
        log!("Running->Jumping");

        RedHatBoyState {
            context: self
                .context
                .set_vertical_velocity(JUMP_SPEED)
                .reset_frame()
                .play_jump_sfx(),
            _state: Jumping {},
        }
    }

    pub fn knock_out(self) -> RedHatBoyState<Falling> {
        log!("Running->Falling");

        RedHatBoyState {
            context: self.context.reset_frame().stop(),
            _state: Falling {},
        }
    }

    pub fn land_on(self, position: i16) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.set_on(position),
            _state: Running,
        }
    }

    pub fn slide(self) -> RedHatBoyState<Sliding> {
        log!("Running->Sliding");

        RedHatBoyState {
            context: self.context.play_slide_sfx().reset_frame(),
            _state: Sliding {},
        }
    }

    pub fn update(mut self) -> Self {
        self.context = self.context.update(RUNNING_FRAMES);

        self
    }
}
