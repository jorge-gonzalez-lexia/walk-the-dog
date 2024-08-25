use crate::engine::Point;

#[derive(Clone, Copy)]
pub struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

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
        if self.frame < frame_count {
            self.frame += 1;
        } else {
            self.frame = 0;
        }

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

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
}

#[derive(Clone, Copy)]
pub struct Idle;

#[derive(Clone, Copy)]
pub struct Running;

#[derive(Clone, Copy)]
pub struct Sliding;

const FLOOR: i16 = 475;

const IDLE_FRAME_NAME: &str = "Idle";
const RUN_FRAME_NAME: &str = "Run";
const SLIDE_FRAME_NAME: &str = "Slide";

const IDLE_FRAMES: u8 = 29;
const RUNNING_FRAMES: u8 = 23;
const SLIDING_FRAMES: u8 = 14;

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

impl RedHatBoyState<Running> {
    pub fn frame_name(&self) -> &str {
        RUN_FRAME_NAME
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

    pub fn update(mut self) -> Self {
        self.context = self.context.update(SLIDING_FRAMES);

        self
    }
}
