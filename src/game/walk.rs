use super::{obstacle::Obstacle, red_hat_boy::RedHatBoy};
use crate::engine::image::Image;

pub struct Walk {
    pub backgrounds: [Image; 2],
    pub boy: RedHatBoy,
    pub obstacles: Vec<Box<dyn Obstacle>>,
}

impl Walk {
    pub fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }
}
