use crate::{
    engine::{
        audio::{Audio, Sound},
        rect::Point,
    },
    game,
};

pub const FLOOR: i16 = 479;
const PLAYER_HEIGHT: i16 = game::HEIGHT - FLOOR;

const GRAVITY: i16 = 1;
const RUNNING_SPEED: i16 = 4;
const TERMINAL_VELOCITY: i16 = 20;

#[derive(Clone)]
pub struct RedHatBoyContext {
    pub frame: u8,
    pub position: Point,
    pub velocity: Point,

    audio: Audio,
    sfx: Sfx,
}

impl RedHatBoyContext {
    pub fn new(audio: Audio, frame: u8, position: Point, sfx: Sfx, velocity: Point) -> Self {
        RedHatBoyContext {
            audio,
            frame,
            position,
            sfx,
            velocity,
        }
    }

    pub fn play_jump_sfx(self) -> Self {
        if let Err(err) = self.audio.play_sound(&self.sfx.jump) {
            log!("Error playing jump sound {:#?}", err);
        }

        self
    }

    pub fn play_ko_sfx(self) -> Self {
        if let Err(err) = self.audio.play_sound(&self.sfx.ko) {
            log!("Error playing knock-out sound {:#?}", err);
        }

        self
    }

    pub fn play_slide_sfx(self) -> Self {
        if let Err(err) = self.audio.play_sound(&self.sfx.slide) {
            log!("Error playing slide sound {:#?}", err);
        }

        self
    }

    pub fn reset_frame(mut self) -> Self {
        self.frame = 0;

        self
    }

    pub fn run_right(mut self) -> Self {
        self.velocity.x += RUNNING_SPEED;

        self
    }

    pub fn set_on(mut self, position: i16) -> Self {
        let position = position - PLAYER_HEIGHT;
        self.position.y = position;

        self
    }

    pub fn set_vertical_velocity(mut self, y: i16) -> Self {
        self.velocity.y = y;

        self
    }

    pub fn stop(mut self) -> Self {
        self.velocity.x = 0;
        self.velocity.y = 0;

        self
    }

    /// Update the frame count or loop back to frame 0 when current frame hits
    ///  `frame_count` (the number of frames in the active state animation)
    pub fn update(mut self, frame_count: u8) -> Self {
        if self.velocity.y < TERMINAL_VELOCITY {
            self.velocity.y += GRAVITY;
        }

        if self.frame < frame_count {
            self.frame += 1;
        } else {
            self.frame = 0;
        }

        self.position.y += self.velocity.y;

        if self.position.y > FLOOR {
            self.position.y = FLOOR;
        }

        self
    }
}

#[derive(Clone)]
pub struct Sfx {
    jump: Sound,
    ko: Sound,
    slide: Sound,
}

impl Sfx {
    pub fn new(jump: Sound, ko: Sound, slide: Sound) -> Self {
        Sfx { jump, ko, slide }
    }
}
