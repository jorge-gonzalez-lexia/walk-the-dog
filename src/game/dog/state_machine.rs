use super::{
    context::DogContext,
    states::{
        fleeing::Fleeing, returning::Returning, returning_to_flee::ReturningToFlee,
        returning_worried::ReturningWorried, running::Running, running_worried::RunningWorried,
        DogState,
    },
};

#[derive(Debug, PartialEq)]
pub enum Event {
    Flee,
    Update,
    Worry,
}

#[derive(Clone)]
pub enum DogStateMachine {
    Fleeing(DogState<Fleeing>),
    Returning(DogState<Returning>),
    ReturningToFlee(DogState<ReturningToFlee>),
    ReturningWorried(DogState<ReturningWorried>),
    Running(DogState<Running>),
    RunningWorried(DogState<RunningWorried>),
}

impl DogStateMachine {
    pub fn context(&self) -> &DogContext {
        match self {
            DogStateMachine::Fleeing(state) => state.context(),
            DogStateMachine::Returning(state) => state.context(),
            DogStateMachine::ReturningToFlee(state) => state.context(),
            DogStateMachine::ReturningWorried(state) => state.context(),
            DogStateMachine::Running(state) => state.context(),
            DogStateMachine::RunningWorried(state) => state.context(),
        }
    }

    pub fn transition(self, event: Event) -> Self {
        if event != Event::Update {
            log!("Dog Event {event:?}");
        }

        match (self.clone(), event) {
            (DogStateMachine::Fleeing(state), Event::Update) => state.update().into(),
            (DogStateMachine::Fleeing(state), Event::Worry) => state.worry().into(),

            (DogStateMachine::Returning(state), Event::Flee) => state.flee().into(),
            (DogStateMachine::Returning(state), Event::Update) => state.update().into(),

            (DogStateMachine::ReturningToFlee(state), Event::Update) => state.update().into(),
            (DogStateMachine::ReturningToFlee(state), Event::Worry) => state.worry().into(),

            (DogStateMachine::Running(state), Event::Flee) => state.flee().into(),
            (DogStateMachine::Running(state), Event::Update) => state.update().into(),

            (DogStateMachine::ReturningWorried(state), Event::Update) => state.update().into(),

            (DogStateMachine::RunningWorried(state), Event::Update) => state.update().into(),

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

impl From<DogState<ReturningToFlee>> for DogStateMachine {
    fn from(state: DogState<ReturningToFlee>) -> Self {
        DogStateMachine::ReturningToFlee(state)
    }
}

impl From<DogState<ReturningWorried>> for DogStateMachine {
    fn from(state: DogState<ReturningWorried>) -> Self {
        DogStateMachine::ReturningWorried(state)
    }
}

impl From<DogState<Running>> for DogStateMachine {
    fn from(state: DogState<Running>) -> Self {
        DogStateMachine::Running(state)
    }
}

impl From<DogState<RunningWorried>> for DogStateMachine {
    fn from(state: DogState<RunningWorried>) -> Self {
        DogStateMachine::RunningWorried(state)
    }
}
