use super::{
    context::DogContext,
    states::{fleeing::Fleeing, returning::Returning, running::Running, DogState},
};

#[derive(Debug, PartialEq)]
pub enum Event {
    Flee,
    Update,
}

#[derive(Clone)]
pub enum DogStateMachine {
    Fleeing(DogState<Fleeing>),
    Returning(DogState<Returning>),
    Running(DogState<Running>),
}

impl DogStateMachine {
    pub fn context(&self) -> &DogContext {
        match self {
            DogStateMachine::Fleeing(state) => state.context(),
            DogStateMachine::Returning(state) => state.context(),
            DogStateMachine::Running(state) => state.context(),
        }
    }

    pub fn transition(self, event: Event) -> Self {
        if event != Event::Update {
            log!("Dog Event {event:?}");
        }

        match (self.clone(), event) {
            (DogStateMachine::Fleeing(state), Event::Update) => state.update().into(),

            (DogStateMachine::Returning(state), Event::Flee) => state.flee().into(),
            (DogStateMachine::Returning(state), Event::Update) => state.update().into(),

            (DogStateMachine::Running(state), Event::Flee) => state.flee().into(),
            (DogStateMachine::Running(state), Event::Update) => state.update().into(),

            _ => self,
        }
    }

    pub fn update(self) -> Self {
        self.transition(Event::Update)
    }
}

impl From<DogState<Fleeing>> for DogStateMachine {
    fn from(state: DogState<Fleeing>) -> Self {
        DogStateMachine::Fleeing(state)
    }
}

impl From<DogState<Returning>> for DogStateMachine {
    fn from(state: DogState<Returning>) -> Self {
        DogStateMachine::Returning(state)
    }
}

impl From<DogState<Running>> for DogStateMachine {
    fn from(state: DogState<Running>) -> Self {
        DogStateMachine::Running(state)
    }
}
