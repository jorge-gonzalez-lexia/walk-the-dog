use super::{
    dog::Dog,
    event_queue::{EventPublisher, EventQueue, EventSubscriber},
    obstacles::Obstacle,
    red_hat_boy::RedHatBoy,
    segments::SegmentFactory,
};
use crate::engine::{image::Image, rect::Point, renderer::Renderer, sprite_sheet::SpriteSheet};
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use web_sys::HtmlImageElement;

const OBSTACLE_BUFFER: i16 = 20;

pub struct Walk {
    pub backgrounds: [Image; 2],
    pub boy: RedHatBoy,
    pub event_publisher: EventPublisher,
    pub obstacles: Vec<Rc<RefCell<Box<dyn Obstacle>>>>,
    pub timeline: i16,

    dog: Rc<RefCell<Dog>>,
    event_subscribers: Vec<Rc<RefCell<dyn EventSubscriber>>>,
    events: EventQueue,
    segment_factory: SegmentFactory,
    stone: HtmlImageElement,
}

impl Walk {
    pub fn new(
        background: HtmlImageElement,
        boy: RedHatBoy,
        dog: Dog,
        event_publisher: EventPublisher,
        events: EventQueue,
        stone: HtmlImageElement,
        segment_tiles: SpriteSheet,
    ) -> Self {
        let mut segment_factory =
            SegmentFactory::new(segment_tiles, stone.clone(), event_publisher.clone());
        let starting_obstacles = into_rc_obstacles(segment_factory.first());
        let timeline = rightmost(&starting_obstacles);

        let mut event_subscribers: Vec<Rc<RefCell<dyn EventSubscriber>>> = Vec::new();
        let dog = Rc::new(RefCell::new(dog));
        event_subscribers.push(dog.clone() as Rc<RefCell<dyn EventSubscriber>>);

        let background_width = background.width() as i16;

        Walk {
            backgrounds: [
                Image::new(background.clone(), Point { x: 0, y: 0 }),
                Image::new(
                    background,
                    Point {
                        x: background_width,
                        y: 0,
                    },
                ),
            ],
            boy,
            dog: dog.clone(),
            events,
            event_publisher,
            event_subscribers,
            obstacles: starting_obstacles,
            segment_factory,
            stone,
            timeline,
        }
    }

    pub fn reset(walk: Self) -> Self {
        let mut segment_factory = walk.segment_factory;
        let starting_obstacles = into_rc_obstacles(segment_factory.first());
        let timeline = rightmost(&starting_obstacles);

        let dog = if let Ok(dog) = Rc::try_unwrap(walk.dog) {
            dog.into_inner()
        } else {
            panic!("Unable to take dog ownership!");
        };

        Walk {
            backgrounds: walk.backgrounds,
            boy: RedHatBoy::reset(walk.boy),
            dog: Rc::new(RefCell::new(Dog::reset(dog))),
            events: walk.events,
            event_publisher: walk.event_publisher,
            event_subscribers: walk.event_subscribers,
            obstacles: starting_obstacles,
            segment_factory,
            stone: walk.stone,
            timeline,
        }
    }

    pub fn dog(&self) -> RefMut<Dog> {
        self.dog.as_ref().borrow_mut()
    }

    pub fn draw(&self, renderer: &Renderer) {
        self.backgrounds.iter().for_each(|b| b.draw(renderer));
        self.boy.draw(renderer);
        self.dog().draw(renderer);
        self.obstacles
            .iter()
            .for_each(|o| o.borrow().draw(renderer));
    }

    pub fn generate_next_segment(&mut self) {
        let offset_x = self.timeline + OBSTACLE_BUFFER;
        let mut next_obstacles = into_rc_obstacles(self.segment_factory.random(offset_x));

        self.timeline = rightmost(&next_obstacles);
        self.obstacles.append(&mut next_obstacles);
    }

    pub fn knocked_out(&self) -> bool {
        self.boy.knocked_out()
    }

    pub fn navigate_obstacles(&mut self) {
        for obstacle in self.obstacles.iter() {
            obstacle.as_ref().borrow_mut().navigate(&self.dog.borrow());
        }
    }

    pub fn process_events(&mut self) {
        while let Some(event) = self.events.as_ref().borrow_mut().pop_front() {
            for o in self.obstacles.iter() {
                o.as_ref().borrow_mut().process_event(&event);
            }
            for s in self.event_subscribers.iter() {
                s.as_ref().borrow_mut().process_event(&event);
            }
        }
    }

    pub fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }
}

fn into_rc_obstacles(obstacles: Vec<Box<dyn Obstacle>>) -> Vec<Rc<RefCell<Box<dyn Obstacle>>>> {
    obstacles
        .into_iter()
        .map(|o| Rc::new(RefCell::new(o)))
        .collect()
}

fn rightmost(obstacle_list: &[Rc<RefCell<Box<dyn Obstacle>>>]) -> i16 {
    obstacle_list
        .iter()
        .map(|o| o.borrow().right())
        .max_by(|x, y| x.cmp(y))
        .unwrap_or(0)
}
