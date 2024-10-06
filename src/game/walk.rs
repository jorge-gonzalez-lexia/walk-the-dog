use super::{
    dog::Dog,
    event_queue::{EventPublisher, EventQueue, EventSubscriber, GameEvent},
    obstacles::{Obstacle, ObstacleVec},
    red_hat_boy::RedHatBoy,
    segments::SegmentFactory,
};
use crate::engine::{image::Image, rect::Point, renderer::Renderer, sprite_sheet::SpriteSheet};
use std::{
    cell::{RefCell, RefMut},
    collections::HashSet,
    rc::Rc,
};
use web_sys::HtmlImageElement;

const OBSTACLE_BUFFER: i16 = 20;

pub struct Walk {
    pub backgrounds: [Image; 2],
    pub boy: RedHatBoy,
    pub event_publisher: EventPublisher,
    pub obstacles: ObstacleVec,
    pub timeline: i16,

    dog: Rc<RefCell<Dog>>,
    event_subscribers: Vec<Rc<RefCell<Box<dyn EventSubscriber>>>>,
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
        let starting_obstacles = segment_factory.first();
        let timeline = rightmost(&starting_obstacles);

        let mut event_subscribers: Vec<Rc<RefCell<Box<dyn EventSubscriber>>>> = Vec::new();
        let dog = Rc::new(RefCell::new(dog));
        let s = DogSubscriber(Rc::clone(&dog));
        event_subscribers.push(Rc::new(RefCell::new(
            Box::new(s) as Box<dyn EventSubscriber>
        )));
        for obstacle in &starting_obstacles {
            let s = ObstacleSubscriber(Rc::clone(obstacle));
            event_subscribers.push(Rc::new(RefCell::new(
                Box::new(s) as Box<dyn EventSubscriber>
            )));
        }

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
            dog,
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
        let starting_obstacles = segment_factory.first();
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

    pub fn drop_surpassed_obstacles(&mut self) {
        let to_drop: HashSet<String> = self
            .obstacles
            .iter()
            .filter(|o| o.borrow().right() <= 0)
            .map(|o| o.borrow().id().to_string())
            .collect();
        self.obstacles
            .retain(|obstacle| obstacle.borrow().right() > 0);
        self.event_subscribers
            .retain(|s| !to_drop.contains(&s.borrow().name()));

        log!(
            "Dropped {} obstacles left behind. Total={} Subscribers={}",
            to_drop.len(),
            self.obstacles.len(),
            self.event_subscribers.len()
        );
    }

    pub fn generate_next_segment(&mut self) {
        let offset_x = self.timeline + OBSTACLE_BUFFER;
        let mut next_obstacles = self.segment_factory.random(offset_x);

        self.timeline = rightmost(&next_obstacles);

        for obstacle in &next_obstacles {
            let s = ObstacleSubscriber(Rc::clone(obstacle));
            self.event_subscribers.push(Rc::new(RefCell::new(
                Box::new(s) as Box<dyn EventSubscriber>
            )));
        }
        let to_add = next_obstacles.len();

        self.obstacles.append(&mut next_obstacles);

        log!(
            "Appended {to_add} obstacles. Total={} Subscribers={}",
            self.obstacles.len(),
            self.event_subscribers.len()
        );
    }

    pub fn knocked_out(&self) -> bool {
        self.boy.knocked_out()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.dog().update();
        self.navigate_obstacles();
    }

    pub fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }

    fn navigate_obstacles(&mut self) {
        for obstacle in self.obstacles.iter() {
            obstacle.as_ref().borrow_mut().navigate(&self.dog.borrow());
        }
    }

    fn process_events(&mut self) {
        while let Some(event) = self.events.as_ref().borrow_mut().pop_front() {
            for s in self.event_subscribers.iter() {
                s.borrow_mut().process_event(&event);
            }
        }
    }
}

fn rightmost(obstacle_list: &[Rc<RefCell<Box<dyn Obstacle>>>]) -> i16 {
    obstacle_list
        .iter()
        .map(|o| o.borrow().right())
        .max_by(|x, y| x.cmp(y))
        .unwrap_or(0)
}

struct DogSubscriber(Rc<RefCell<Dog>>);
impl EventSubscriber for DogSubscriber {
    fn name(&self) -> String {
        self.0.borrow().name()
    }

    fn process_event(&mut self, event: &GameEvent) {
        self.0.borrow_mut().process_event(event);
    }
}

struct ObstacleSubscriber(Rc<RefCell<Box<dyn Obstacle>>>);
impl EventSubscriber for ObstacleSubscriber {
    fn name(&self) -> String {
        self.0.borrow().id()
    }

    fn process_event(&mut self, event: &GameEvent) {
        self.0.borrow_mut().process_event(event);
    }
}
