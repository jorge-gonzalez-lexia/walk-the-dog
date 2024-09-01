use super::{running::Running, RedHatBoyState};
use crate::{
    engine::{
        audio::{Audio, Sound},
        rect::Point,
    },
    game::red_hat_boy::context::{self, RedHatBoyContext},
};

const STARTING_POINT: i16 = -20;

#[derive(Clone, Copy)]
pub struct Idle;

const IDLE_FRAME_NAME: &str = "Idle";
const IDLE_FRAMES: u8 = 29;

impl RedHatBoyState<Idle> {
    pub fn new(audio: Audio, sfx_jump: Sound, sfx_ko: Sound) -> Self {
        RedHatBoyState {
            context: RedHatBoyContext {
                audio,
                frame: 0,
                sfx_jump,
                sfx_ko,
                position: Point {
                    x: STARTING_POINT,
                    y: context::FLOOR,
                },
                velocity: Point { x: 0, y: 0 },
            },
            _state: Idle {},
        }
    }

    pub fn frame_name(&self) -> &str {
        IDLE_FRAME_NAME
    }

    pub fn run(self) -> RedHatBoyState<Running> {
        log!("Idle->Running");

        RedHatBoyState {
            context: self.context.reset_frame().run_right(),
            _state: Running {},
        }
    }

    pub fn update(mut self) -> Self {
        self.context = self.context.update(IDLE_FRAMES);

        self
    }
}
