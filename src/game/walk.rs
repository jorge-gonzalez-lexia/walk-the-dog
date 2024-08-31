use super::{obstacle::Obstacle, red_hat_boy::RedHatBoy};
use crate::engine::{image::Image, sprite_sheet::SpriteSheet};
use std::rc::Rc;
use web_sys::HtmlImageElement;

pub struct Walk {
    pub backgrounds: [Image; 2],
    pub boy: RedHatBoy,
    pub obstacle_sheet: Rc<SpriteSheet>,
    pub obstacles: Vec<Box<dyn Obstacle>>,
    pub stone: HtmlImageElement,
    pub timeline: i16,
}

impl Walk {
    pub fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }
}
