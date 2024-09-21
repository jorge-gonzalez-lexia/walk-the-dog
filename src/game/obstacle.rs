use crate::engine::renderer::Renderer;

use super::{dog::Dog, red_hat_boy::RedHatBoy};

pub trait Obstacle {
    fn check_intersection(&self, boy: &mut RedHatBoy);
    fn draw(&self, renderer: &Renderer);
    fn move_horizontally(&mut self, x: i16);
    fn navigate(&self, dog: &mut Dog);
    fn right(&self) -> i16;
}
