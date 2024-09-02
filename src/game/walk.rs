use super::{
    obstacle::Obstacle,
    red_hat_boy::RedHatBoy,
    segments::{
        platform_and_stone, platform_high, platform_low, stone, stone_and_platform,
        stone_on_platform,
    },
};
use crate::engine::{image::Image, renderer::Renderer, sprite_sheet::SpriteSheet};
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
    pub fn reset(walk: Self) -> Self {
        let starting_obstacles =
            stone_and_platform(walk.stone.clone(), walk.obstacle_sheet.clone(), 0);
        let timeline = rightmost(&starting_obstacles);

        Walk {
            backgrounds: walk.backgrounds,
            boy: RedHatBoy::reset(walk.boy),
            obstacle_sheet: walk.obstacle_sheet,
            obstacles: starting_obstacles,
            stone: walk.stone,
            timeline,
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        self.backgrounds.iter().for_each(|b| b.draw(renderer));
        self.boy.draw(renderer);
        self.obstacles.iter().for_each(|o| o.draw(renderer));
    }

    pub fn generate_next_segment(&mut self) {
        let obstacle_sheet = self.obstacle_sheet.clone();
        let offset_x = self.timeline + OBSTACLE_BUFFER;

        let mut rng = thread_rng();
        let next_segment = rng.gen_range(0..5);
        let mut next_obstacles = match next_segment {
            0 => platform_and_stone(self.stone.clone(), obstacle_sheet, offset_x),
            1 => platform_high(obstacle_sheet, offset_x),
            2 => platform_low(obstacle_sheet, offset_x),
            3 => stone(self.stone.clone(), offset_x),
            4 => stone_and_platform(self.stone.clone(), obstacle_sheet, offset_x),
            5 => stone_on_platform(self.stone.clone(), obstacle_sheet, offset_x),

            _ => vec![],
        };

        self.timeline = rightmost(&next_obstacles);
        self.obstacles.append(&mut next_obstacles);
    }

    pub fn knocked_out(&self) -> bool {
        self.boy.knocked_out()
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
