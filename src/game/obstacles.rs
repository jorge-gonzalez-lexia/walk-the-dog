pub mod barrier;
pub mod obstacle_mark;
pub mod platform;

use super::{dog::Dog, event_queue::EventSubscriber, red_hat_boy::RedHatBoy};
use crate::engine::renderer::Renderer;
use obstacle_mark::ObstacleMark;

pub trait Obstacle: EventSubscriber {
    fn check_intersection(&self, boy: &mut RedHatBoy);
    fn draw(&self, renderer: &Renderer);
    fn move_horizontally(&mut self, x: i16);
    fn navigate(&mut self, dog: &Dog);
    fn right(&self) -> i16;
}

pub trait ObstacleMarkFactory {
    // Dog navigation (jump point) mark left of the obstacle
    fn mark_left(&self) -> ObstacleMark;
    // Dog navigation (jump point) mark right of the obstacle
    fn mark_right(&self) -> ObstacleMark;
}
