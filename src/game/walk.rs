use super::{platform::Platform, red_hat_boy::RedHatBoy};
use crate::engine::image::Image;

pub struct Walk {
    pub backgrounds: [Image; 2],
    pub boy: RedHatBoy,
    pub platform: Platform,
    pub stone: Image,
}

impl Walk {
    pub fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }
}
