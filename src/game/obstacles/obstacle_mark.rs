use super::Obstacle;
use crate::{
    engine::rect::{Point, Rect},
    game::{
        event_queue::{EventPublisher, GameEvent},
        red_hat_boy::RedHatBoy,
        HEIGHT,
    },
};

pub enum ObstacleMarkDirection {
    Left,
    Right,
}

/// A jump mark for the Dog to navigate past an obstacle
pub struct ObstacleMark {
    direction: ObstacleMarkDirection,
    event_publisher: EventPublisher,
    // Dog intersecting w/ mark (on last update loop iteration)
    has_dog: bool,
    id: String,
    position: Point,
}

impl ObstacleMark {
    pub fn new(
        position: Point,
        direction: ObstacleMarkDirection,
        obstacle_id: String,
        event_publisher: EventPublisher,
    ) -> Self {
        let dir = match direction {
            ObstacleMarkDirection::Left => "l",
            ObstacleMarkDirection::Right => "r",
        };

        ObstacleMark {
            id: format!("{}_{dir}m", obstacle_id),
            direction,
            event_publisher,
            has_dog: false,
            position,
        }
    }

    fn mark(&self) -> Rect {
        Rect::new(self.position, 1, HEIGHT - self.position.y)
    }
}

impl Obstacle for ObstacleMark {
    fn check_intersection(&self, _boy: &mut RedHatBoy) {}

    fn draw(&self, renderer: &crate::engine::renderer::Renderer) {
        let color = match self.direction {
            ObstacleMarkDirection::Left => "#000000",
            ObstacleMarkDirection::Right => "#FFFF00",
        };

        renderer.draw_rect_colored(&self.mark(), color);
    }

    fn move_horizontally(&mut self, x: i16) {
        self.position.x += x;
    }

    fn navigate(&mut self, dog: &crate::game::dog::Dog) {
        let is_relevant = matches!(self.direction, ObstacleMarkDirection::Left)
            && dog.moving_right()
            || matches!(self.direction, ObstacleMarkDirection::Right) && dog.moving_left();

        let is_on_mark = is_relevant && dog.bounding_box().intersects(&self.mark());

        if is_on_mark && !self.has_dog {
            self.event_publisher.publish(GameEvent::DogHitMark {
                id: self.id.clone(),
            });
        }

        if !is_on_mark && self.has_dog {
            self.event_publisher.publish(GameEvent::DogOffMark {
                id: self.id.clone(),
            });
        }
    }

    fn process_event(&mut self, event: &crate::game::event_queue::GameEvent) {
        match event {
            GameEvent::DogHitMark { id } if *id == self.id => {
                log!("Mark {}: Dog hit mark", self.id);
                self.has_dog = true;
            }
            GameEvent::DogOffMark { id } if *id == self.id => {
                log!("Mark {}: Dog off mark", self.id);
                self.has_dog = false;
            }
            _ => (),
        }
    }

    fn right(&self) -> i16 {
        self.position.x + 1
    }
}
