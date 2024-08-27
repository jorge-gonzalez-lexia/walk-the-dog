use crate::engine::Point;

use super::RedHatBoyStateMachine;

#[derive(Clone, Copy)]
pub struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

const GRAVITY: i16 = 1;
const RUNNING_SPEED: i16 = 3;

#[derive(Clone, Copy)]
pub struct RedHatBoyContext {
    pub frame: u8,
    pub position: Point,
    pub velocity: Point,
}

impl RedHatBoyContext {
    /// Update the frame count or loop back to frame 0 when current frame hits
    ///  `frame_count` (the number of frames in the active state animation)
    pub fn update(mut self, frame_count: u8) -> Self {
        self.velocity.y += GRAVITY;
        if self.frame < frame_count {
            self.frame += 1;
        } else {
            self.frame = 0;
        }

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        if self.position.y > FLOOR {
            self.position.y = FLOOR;
        }

        self
    }

    fn reset_frame(mut self) -> Self {
        self.frame = 0;

        self
    }

    fn run_right(mut self) -> Self {
        self.velocity.x += RUNNING_SPEED;

        self
    }

    fn set_vertical_velocity(mut self, y: i16) -> Self {
        self.velocity.y = y;

        self
    }
}

#[derive(Clone, Copy)]
pub struct Idle;

#[derive(Clone, Copy)]
pub struct Jumping;

#[derive(Clone, Copy)]
pub struct Running;

#[derive(Clone, Copy)]
pub struct Sliding;

const FLOOR: i16 = 475;

const IDLE_FRAME_NAME: &str = "Idle";
const JUMP_FRAME_NAME: &str = "Jump";
const RUN_FRAME_NAME: &str = "Run";
const SLIDE_FRAME_NAME: &str = "Slide";

const IDLE_FRAMES: u8 = 29;
const JUMPING_FRAMES: u8 = 35;
const RUNNING_FRAMES: u8 = 23;
const SLIDING_FRAMES: u8 = 14;

impl<S> RedHatBoyState<S> {
    pub fn context(&self) -> &RedHatBoyContext {
        &self.context
    }
}

const STARTING_POINT: i16 = -20;

impl RedHatBoyState<Idle> {
    pub fn new() -> Self {
        RedHatBoyState {
            context: RedHatBoyContext {
                frame: 0,
                position: Point {
                    x: STARTING_POINT,
                    y: FLOOR,
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

impl RedHatBoyState<Jumping> {
    pub fn frame_name(&self) -> &str {
        JUMP_FRAME_NAME
    }

    pub fn update(mut self) -> JumpingEndState {
        self.context = self.context.update(JUMPING_FRAMES);

        if self.context.position.y >= FLOOR {
            JumpingEndState::Complete(self.land())
        } else {
            JumpingEndState::Jumping(self)
        }
    }

    fn land(self) -> RedHatBoyState<Running> {
        log!("Jumping->Running");

        RedHatBoyState {
            context: self.context.reset_frame(),
            _state: Running {},
        }
    }
}

pub enum JumpingEndState {
    Complete(RedHatBoyState<Running>),
    Jumping(RedHatBoyState<Jumping>),
}

impl From<JumpingEndState> for RedHatBoyStateMachine {
    fn from(end_state: JumpingEndState) -> Self {
        match end_state {
            JumpingEndState::Complete(running_state) => running_state.into(),
            JumpingEndState::Jumping(jumping_state) => jumping_state.into(),
        }
    }
}

const JUMP_SPEED: i16 = -25;

impl RedHatBoyState<Running> {
    pub fn frame_name(&self) -> &str {
        RUN_FRAME_NAME
    }

    pub fn jump(self) -> RedHatBoyState<Jumping> {
        log!("Running->Jumping");

        RedHatBoyState {
            context: self.context.set_vertical_velocity(JUMP_SPEED).reset_frame(),
            _state: Jumping {},
        }
    }

    pub fn slide(self) -> RedHatBoyState<Sliding> {
        log!("Running->Sliding");

        RedHatBoyState {
            context: self.context.reset_frame(),
            _state: Sliding {},
        }
    }

    pub fn update(mut self) -> Self {
        self.context = self.context.update(RUNNING_FRAMES);

        self
    }
}

impl RedHatBoyState<Sliding> {
    pub fn frame_name(&self) -> &str {
        SLIDE_FRAME_NAME
    }

    pub fn update(mut self) -> SlidingEndState {
        self.context = self.context.update(SLIDING_FRAMES);

        if self.context.frame >= SLIDING_FRAMES {
            SlidingEndState::Complete(self.stand())
        } else {
            SlidingEndState::Sliding(self)
        }
    }

    fn stand(self) -> RedHatBoyState<Running> {
        log!("Sliding->Running");

        RedHatBoyState {
            context: self.context().reset_frame(),
            _state: Running {},
        }
    }
}

pub enum SlidingEndState {
    Complete(RedHatBoyState<Running>),
    Sliding(RedHatBoyState<Sliding>),
}

impl From<SlidingEndState> for RedHatBoyStateMachine {
    fn from(end_state: SlidingEndState) -> Self {
        match end_state {
            SlidingEndState::Complete(running_state) => running_state.into(),
            SlidingEndState::Sliding(sliding_state) => sliding_state.into(),
        }
    }
}
