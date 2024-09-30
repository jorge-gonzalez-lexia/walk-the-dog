use super::{
    dog::Dog,
    event_queue::{EventPublisher, EventQueue},
    obstacle::Obstacle,
    red_hat_boy::RedHatBoy,
    segments::SegmentFactory,
};
use crate::engine::{image::Image, renderer::Renderer};
use web_sys::HtmlImageElement;

const OBSTACLE_BUFFER: i16 = 20;

pub struct Walk {
    pub backgrounds: [Image; 2],
    pub boy: RedHatBoy,
    pub dog: Dog,
    pub events: EventQueue,
    pub event_publisher: EventPublisher,
    pub obstacles: Vec<Box<dyn Obstacle>>,
    pub segment_factory: SegmentFactory,
    pub stone: HtmlImageElement,
    pub timeline: i16,
}

impl Walk {
    pub fn reset(walk: Self) -> Self {
        let mut segment_factory = walk.segment_factory;
        let starting_obstacles = segment_factory.first();
        let timeline = rightmost(&starting_obstacles);

        Walk {
            backgrounds: walk.backgrounds,
            boy: RedHatBoy::reset(walk.boy),
            dog: Dog::reset(walk.dog),
            events: walk.events,
            event_publisher: walk.event_publisher,
            obstacles: starting_obstacles,
            segment_factory,
            stone: walk.stone,
            timeline,
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        self.backgrounds.iter().for_each(|b| b.draw(renderer));
        self.boy.draw(renderer);
        self.dog.draw(renderer);
        self.obstacles.iter().for_each(|o| o.draw(renderer));
    }

    pub fn generate_next_segment(&mut self) {
        let offset_x = self.timeline + OBSTACLE_BUFFER;
        let mut next_obstacles = self.segment_factory.random(offset_x);

        self.timeline = rightmost(&next_obstacles);
        self.obstacles.append(&mut next_obstacles);
    }

    pub fn knocked_out(&self) -> bool {
        self.boy.knocked_out()
    }

    pub fn process_events(&mut self) {
        while let Some(event) = self.events.borrow_mut().pop_front() {
            self.obstacles.iter_mut().for_each(|o| {
                o.process_event(&event);
            });
            self.dog.process_event(&event);
        }
    }

    pub fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }
}

pub fn rightmost(obstacle_list: &[Box<dyn Obstacle>]) -> i16 {
    obstacle_list
        .iter()
        .map(|o| o.right())
        .max_by(|x, y| x.cmp(y))
        .unwrap_or(0)
}
