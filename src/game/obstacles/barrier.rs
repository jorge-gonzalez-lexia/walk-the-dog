use super::{
    obstacle_mark::{ObstacleMark, ObstacleMarkDirection},
    Obstacle, ObstacleMarkFactory,
};
use crate::{
    engine::{image::Image, rect::Point, renderer::Renderer},
    game::{
        dog::Dog,
        event_queue::{EventPublisher, GameEvent},
        red_hat_boy::RedHatBoy,
    },
};

pub struct Barrier {
    event_publisher: EventPublisher,
    id: String,
    image: Image,
}

impl Barrier {
    pub fn new(id: String, image: Image, event_publisher: EventPublisher) -> Self {
        Barrier {
            event_publisher,
            id,
            image,
        }
    }
}

impl Obstacle for Barrier {
    fn check_intersection(&self, boy: &mut RedHatBoy) {
        if boy.bounding_box().intersects(self.image.bounding_box()) {
            boy.knock_out();
            self.event_publisher.publish(GameEvent::BoyHitsObstacle);
        }
    }

    fn draw(&self, renderer: &Renderer) {
        self.image.draw(renderer);
    }

    fn move_horizontally(&mut self, x: i16) {
        self.image.move_horizontally(x);
    }

    fn navigate(&mut self, _dog: &Dog) {}

    fn process_event(&mut self, _event: &GameEvent) {}

    fn right(&self) -> i16 {
        self.image.right()
    }
}

impl ObstacleMarkFactory for Barrier {
    fn mark_left(&self) -> ObstacleMark {
        ObstacleMark::new(
            Point {
                x: self.image.bounding_box().x() - 20,
                y: self.image.bounding_box().y(),
            },
            ObstacleMarkDirection::Left,
            self.id.clone(),
            self.event_publisher.clone(),
        )
    }

    fn mark_right(&self) -> ObstacleMark {
        ObstacleMark::new(
            Point {
                x: self.image.bounding_box().right() + 20,
                y: self.image.bounding_box().y(),
            },
            ObstacleMarkDirection::Right,
            self.id.clone(),
            self.event_publisher.clone(),
        )
    }
}
