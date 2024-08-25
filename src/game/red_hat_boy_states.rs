use crate::engine::Point;

#[derive(Clone, Copy)]
pub struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

#[derive(Clone, Copy)]
pub struct RedHatBoyContext {
    pub frame: u8,
    pub position: Point,
    pub velocity: Point,
}

#[derive(Clone, Copy)]
pub struct Idle;

#[derive(Clone, Copy)]
pub struct Running;

const FLOOR: i16 = 475;
const IDLE_FRAME_NAME: &str = "Idle";
const RUN_FRAME_NAME: &str = "Run";

impl<S> RedHatBoyState<S> {
    pub fn context(&self) -> &RedHatBoyContext {
        &self.context
    }
}

impl RedHatBoyState<Idle> {
    pub fn new() -> Self {
        RedHatBoyState {
            context: RedHatBoyContext {
                frame: 0,
                position: Point { x: 0, y: FLOOR },
                velocity: Point { x: 0, y: 0 },
            },
            _state: Idle {},
        }
    }

    pub fn frame_name(&self) -> &str {
        IDLE_FRAME_NAME
    }

    pub fn run(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context,
            _state: Running {},
        }
    }
}

impl RedHatBoyState<Running> {
    pub fn frame_name(&self) -> &str {
        RUN_FRAME_NAME
    }
}
