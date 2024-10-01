use crate::engine::renderer::Renderer;

use super::{dog::Dog, event_queue::GameEvent, red_hat_boy::RedHatBoy};

pub trait Obstacle {
    fn check_intersection(&self, boy: &mut RedHatBoy);
    fn draw(&self, renderer: &Renderer);
    fn move_horizontally(&mut self, x: i16);
    fn navigate(&mut self, dog: &Dog);
    fn process_event(&mut self, event: &GameEvent);
    fn right(&self) -> i16;
}
