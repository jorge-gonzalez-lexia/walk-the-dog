use super::{obstacle::Obstacle, red_hat_boy::RedHatBoy};
use crate::engine::image::Image;

pub struct Walk {
    pub backgrounds: [Image; 2],
    pub boy: RedHatBoy,
    pub platform: Box<dyn Obstacle>,
    pub stone: Image,
}

impl Walk {
    pub fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }
}
