use super::{
    obstacle::Obstacle,
    red_hat_boy::RedHatBoy,
    segments::{platform_and_stone, stone_and_platform},
};
use crate::engine::{image::Image, sprite_sheet::SpriteSheet};
use rand::{thread_rng, Rng};
use std::rc::Rc;
use web_sys::HtmlImageElement;

const OBSTACLE_BUFFER: i16 = 20;

pub struct Walk {
    pub backgrounds: [Image; 2],
    pub boy: RedHatBoy,
    pub obstacle_sheet: Rc<SpriteSheet>,
    pub obstacles: Vec<Box<dyn Obstacle>>,
    pub stone: HtmlImageElement,
    pub timeline: i16,
}

impl Walk {
    pub fn generate_next_segment(&mut self) {
        let mut rng = thread_rng();
        let next_segment = rng.gen_range(0..2);
        let mut next_obstacles = match next_segment {
            0 => stone_and_platform(
                self.stone.clone(),
                self.obstacle_sheet.clone(),
                self.timeline + OBSTACLE_BUFFER,
            ),
            1 => platform_and_stone(
                self.stone.clone(),
                self.obstacle_sheet.clone(),
                self.timeline + OBSTACLE_BUFFER,
            ),

            _ => vec![],
        };

        self.timeline = rightmost(&next_obstacles);
        self.obstacles.append(&mut next_obstacles);
    }

    pub fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }
}

pub fn rightmost(obstacle_list: &Vec<Box<dyn Obstacle>>) -> i16 {
    obstacle_list
        .iter()
        .map(|o| o.right())
        .max_by(|x, y| x.cmp(&y))
        .unwrap_or(0)
}
