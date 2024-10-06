use std::{cell::RefCell, collections::VecDeque, rc::Rc};

/// Used by game objects to publish (dispatch) GameEvents
#[derive(Clone, Debug)]
pub struct EventPublisher {
    events: EventQueue,
}

impl EventPublisher {
    pub fn new(events: EventQueue) -> Self {
        EventPublisher { events }
    }

    pub fn publish(&self, event: GameEvent) {
        self.events.borrow_mut().push_back(event);
    }
}

/// Queue shared between the `EventPublisher` (which is shared among many game objects)
/// and `Walk`, which deques `GameEvent`s and notifies relevant game objects
pub type EventQueue = Rc<RefCell<VecDeque<GameEvent>>>;

pub trait EventSubscriber {
    fn name(&self) -> String;
    fn process_event(&mut self, event: &GameEvent);
}

#[derive(Debug)]
pub enum GameEvent {
    BoyHitsObstacle,
    DogExitsPlatform,
    DogHitMark { id: String },
    DogOffMark { id: String },
    DogLandedOnGround,
    DogLandedOnPlatform { id: String, platform_top: i16 },
    DogTooClose,
    DogTooFar,
    GameStarted,
}
