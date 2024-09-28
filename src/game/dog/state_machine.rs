use super::{
    context::DogContext,
    states::{jumping::Jumping, running::Running, DogState},
};

#[derive(Debug, PartialEq)]
pub enum Event {
    Flee,
    Jump,
    Land(i16),
    OffPlatform,
    Update,
    Worry,
}

#[derive(Clone)]
pub enum DogStateMachine {
    Jumping(DogState<Jumping>),
    Running(DogState<Running>),
}

impl DogStateMachine {
    pub fn context(&self) -> &DogContext {
        match self {
            DogStateMachine::Jumping(state) => state.context(),
            DogStateMachine::Running(state) => state.context(),
        }
    }

    pub fn state_name(&self) -> &'static str {
        match self {
            DogStateMachine::Jumping(_) => "Jumping",
            DogStateMachine::Running(_) => "Running",
        }
    }

    pub fn transition(self, event: Event) -> Self {
        if event != Event::Update {
            log!("Dog Event '{event:?}' in state '{}'", self.state_name());
        }

        match (self.clone(), event) {
            (DogStateMachine::Jumping(state), Event::Flee) => state.flee().into(),
            (DogStateMachine::Jumping(state), Event::Jump) => state.into(), // explicitly ignore
            (DogStateMachine::Jumping(state), Event::Land(p)) => state.land_on(p).into(),
            (DogStateMachine::Jumping(state), Event::Update) => state.update().into(),
            (DogStateMachine::Jumping(state), Event::Worry) => state.worry().into(),

            (DogStateMachine::Running(state), Event::Flee) => state.flee().into(),
            (DogStateMachine::Running(state), Event::Jump) => state.jump().into(),
            (DogStateMachine::Running(state), Event::OffPlatform) => {
                state.drop_from_platform().into()
            }
            (DogStateMachine::Running(state), Event::Update) => state.update().into(),
            (DogStateMachine::Running(state), Event::Worry) => state.worry().into(),

            (s, event) => {
                error!(
                    "Dog: unhandled event '{:?}' for state '{}'",
                    event,
                    s.state_name()
                );
                panic!("Unhandled event");
            }
        }
    }

    pub fn update(self) -> Self {
        self.transition(Event::Update)
    }
}

impl From<DogState<Jumping>> for DogStateMachine {
    fn from(state: DogState<Jumping>) -> Self {
        DogStateMachine::Jumping(state)
    }
}

impl From<DogState<Running>> for DogStateMachine {
    fn from(state: DogState<Running>) -> Self {
        DogStateMachine::Running(state)
    }
}
