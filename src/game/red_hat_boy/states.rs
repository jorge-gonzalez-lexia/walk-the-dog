use crate::{
    engine::{
        audio::{Audio, Sound},
        rect::Point,
    },
    game::HEIGHT,
};

use super::{
    context::{self, RedHatBoyContext},
    state_machine::RedHatBoyStateMachine,
};

#[derive(Clone)]
pub struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

#[derive(Clone, Copy)]
pub struct Idle;

#[derive(Clone, Copy)]
pub struct Jumping;

#[derive(Clone, Copy)]
pub struct Falling;

#[derive(Clone, Copy)]
pub struct KnockedOut;

#[derive(Clone, Copy)]
pub struct Running;

#[derive(Clone, Copy)]
pub struct Sliding;

const FALL_FRAME_NAME: &str = "Dead";
const IDLE_FRAME_NAME: &str = "Idle";
const JUMP_FRAME_NAME: &str = "Jump";
const DEAD_FRAME_NAME: &str = "Dead";
const RUN_FRAME_NAME: &str = "Run";
const SLIDE_FRAME_NAME: &str = "Slide";

// frames = (animation frame count * 3) -1
const IDLE_FRAMES: u8 = 29;
const JUMPING_FRAMES: u8 = 35;
const FALLING_FRAMES: u8 = 29;
const RUNNING_FRAMES: u8 = 23;
const SLIDING_FRAMES: u8 = 14;

impl<S> RedHatBoyState<S> {
    pub fn context(&self) -> &RedHatBoyContext {
        &self.context
    }
}

const STARTING_POINT: i16 = -20;

impl RedHatBoyState<Falling> {
    pub fn frame_name(&self) -> &str {
        FALL_FRAME_NAME
    }

    pub fn update(mut self) -> FallingEndState {
        self.context = self.context.update(FALLING_FRAMES);

        if self.context.frame >= FALLING_FRAMES {
            FallingEndState::Complete(self.dead())
        } else {
            FallingEndState::Falling(self)
        }
    }

    fn dead(self) -> RedHatBoyState<KnockedOut> {
        log!("Falling->KnockedOut");

        RedHatBoyState {
            context: self.context.play_ko_sfx(),
            _state: KnockedOut {},
        }
    }
}

pub enum FallingEndState {
    Complete(RedHatBoyState<KnockedOut>),
    Falling(RedHatBoyState<Falling>),
}

impl From<FallingEndState> for RedHatBoyStateMachine {
    fn from(end_state: FallingEndState) -> Self {
        match end_state {
            FallingEndState::Complete(knocked_out_state) => knocked_out_state.into(),
            FallingEndState::Falling(falling_state) => falling_state.into(),
        }
    }
}

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

impl RedHatBoyState<Jumping> {
    pub fn frame_name(&self) -> &str {
        JUMP_FRAME_NAME
    }

    pub fn knock_out(self) -> RedHatBoyState<Falling> {
        log!("Jumping->Falling");

        RedHatBoyState {
            context: self.context.reset_frame().stop(),
            _state: Falling {},
        }
    }

    pub fn land_on(self, position: i16) -> RedHatBoyState<Running> {
        log!("Jumping->Running");

        RedHatBoyState {
            context: self.context.reset_frame().set_on(position),
            _state: Running,
        }
    }

    pub fn update(mut self) -> JumpingEndState {
        self.context = self.context.update(JUMPING_FRAMES);

        if self.context.position.y >= context::FLOOR {
            JumpingEndState::Landing(self.land_on(HEIGHT.into()))
        } else {
            JumpingEndState::Jumping(self)
        }
    }
}

pub enum JumpingEndState {
    Jumping(RedHatBoyState<Jumping>),
    Landing(RedHatBoyState<Running>),
}

impl From<JumpingEndState> for RedHatBoyStateMachine {
    fn from(end_state: JumpingEndState) -> Self {
        match end_state {
            JumpingEndState::Landing(running_state) => running_state.into(),
            JumpingEndState::Jumping(jumping_state) => jumping_state.into(),
        }
    }
}

impl RedHatBoyState<KnockedOut> {
    pub fn frame_name(&self) -> &str {
        DEAD_FRAME_NAME
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

    pub fn knock_out(self) -> RedHatBoyState<Falling> {
        log!("Sliding->Falling");

        RedHatBoyState {
            context: self.context().clone().reset_frame().stop(),
            _state: Falling {},
        }
    }

    pub fn land_on(self, position: i16) -> RedHatBoyState<Sliding> {
        RedHatBoyState {
            context: self.context.set_on(position),
            _state: Sliding,
        }
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
            context: self.context().clone().reset_frame(),
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
