use std::{cell::RefCell, collections::VecDeque, rc::Rc};

/// Used by game objects to publish (dispatch) GameEvents
#[derive(Clone)]
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

#[derive(Debug)]
pub enum GameEvent {
    DogTooClose,
    DogTooFar,
}
