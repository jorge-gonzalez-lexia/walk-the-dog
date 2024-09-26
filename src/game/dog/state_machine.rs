use super::{
    context::DogContext,
    states::{
        jumping::Jumping, jumping_worried::JumpingWorried,
        jumping_worried_return::JumpingWorriedReturn, returning_worried::ReturningWorried,
        running::Running, running_worried::RunningWorried, DogState,
    },
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
    JumpingWorried(DogState<JumpingWorried>),
    JumpingWorriedReturn(DogState<JumpingWorriedReturn>),
    ReturningWorried(DogState<ReturningWorried>),
    Running(DogState<Running>),
    RunningWorried(DogState<RunningWorried>),
}

impl DogStateMachine {
    pub fn context(&self) -> &DogContext {
        match self {
            DogStateMachine::Jumping(state) => state.context(),
            DogStateMachine::JumpingWorried(state) => state.context(),
            DogStateMachine::JumpingWorriedReturn(state) => state.context(),
            DogStateMachine::ReturningWorried(state) => state.context(),
            DogStateMachine::Running(state) => state.context(),
            DogStateMachine::RunningWorried(state) => state.context(),
        }
    }

    pub fn state_name(&self) -> &'static str {
        match self {
            DogStateMachine::Jumping(_) => "Jumping",
            DogStateMachine::JumpingWorried(_) => "JumpingWorried",
            DogStateMachine::JumpingWorriedReturn(_) => "JumpingWorriedReturn",
            DogStateMachine::ReturningWorried(_) => "ReturningWorried",
            DogStateMachine::Running(_) => "Running",
            DogStateMachine::RunningWorried(_) => "RunningWorried",
        }
    }

    pub fn transition(self, event: Event) -> Self {
        if event != Event::Update {
            log!("Dog Event '{event:?}' in state '{}'", self.state_name());
        }

        match (self.clone(), event) {
            (DogStateMachine::Jumping(state), Event::Land(p)) => state.land_on(p).into(),
            (DogStateMachine::Jumping(state), Event::Update) => state.update().into(),

            (DogStateMachine::JumpingWorried(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::JumpingWorried(state), Event::Update) => state.update().into(),

            (DogStateMachine::JumpingWorriedReturn(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::JumpingWorriedReturn(state), Event::Update) => state.update().into(),

            (DogStateMachine::ReturningWorried(state), Event::Jump) => state.jump().into(),
            (DogStateMachine::ReturningWorried(state), Event::Update) => state.update().into(),

            (DogStateMachine::Running(state), Event::Flee) => state.flee().into(),
            (DogStateMachine::Running(state), Event::Jump) => state.jump().into(),
            (DogStateMachine::Running(state), Event::OffPlatform) => {
                state.drop_from_platform().into()
            }
            (DogStateMachine::Running(state), Event::Update) => state.update().into(),

            (DogStateMachine::RunningWorried(state), Event::Jump) => state.jump().into(),
            (DogStateMachine::RunningWorried(state), Event::OffPlatform) => {
                state.drop_from_platform().into()
            }
            (DogStateMachine::RunningWorried(state), Event::Update) => state.update().into(),

            (s, event) => {
                error!(
                    "Dog: unhandled event '{:?}' for state '{}'",
                    event,
                    s.state_name()
                );
                self
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

impl From<DogState<JumpingWorried>> for DogStateMachine {
    fn from(state: DogState<JumpingWorried>) -> Self {
        DogStateMachine::JumpingWorried(state)
    }
}

impl From<DogState<JumpingWorriedReturn>> for DogStateMachine {
    fn from(state: DogState<JumpingWorriedReturn>) -> Self {
        DogStateMachine::JumpingWorriedReturn(state)
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
