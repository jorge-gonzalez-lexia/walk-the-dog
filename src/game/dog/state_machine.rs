use super::{
    context::DogContext,
    states::{
        fleeing::Fleeing, jumping::Jumping, jumping_flee::JumpingFlee,
        jumping_flee_return::JumpingFleeReturn, jumping_return::JumpingReturn,
        jumping_worried::JumpingWorried, jumping_worried_return::JumpingWorriedReturn,
        returning::Returning, returning_to_flee::ReturningToFlee,
        returning_worried::ReturningWorried, running::Running, running_worried::RunningWorried,
        DogState,
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
    Fleeing(DogState<Fleeing>),
    Jumping(DogState<Jumping>),
    JumpingFlee(DogState<JumpingFlee>),
    JumpingFleeReturn(DogState<JumpingFleeReturn>),
    JumpingReturn(DogState<JumpingReturn>),
    JumpingWorried(DogState<JumpingWorried>),
    JumpingWorriedReturn(DogState<JumpingWorriedReturn>),
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
            DogStateMachine::Jumping(state) => state.context(),
            DogStateMachine::JumpingFlee(state) => state.context(),
            DogStateMachine::JumpingFleeReturn(state) => state.context(),
            DogStateMachine::JumpingReturn(state) => state.context(),
            DogStateMachine::JumpingWorried(state) => state.context(),
            DogStateMachine::JumpingWorriedReturn(state) => state.context(),
            DogStateMachine::Returning(state) => state.context(),
            DogStateMachine::ReturningToFlee(state) => state.context(),
            DogStateMachine::ReturningWorried(state) => state.context(),
            DogStateMachine::Running(state) => state.context(),
            DogStateMachine::RunningWorried(state) => state.context(),
        }
    }

    pub fn state_name(&self) -> &'static str {
        match self {
            DogStateMachine::Fleeing(_) => "Fleeing",
            DogStateMachine::Jumping(_) => "Jumping",
            DogStateMachine::JumpingFlee(_) => "JumpingFlee",
            DogStateMachine::JumpingFleeReturn(_) => "JumpingFleeReturn",
            DogStateMachine::JumpingReturn(_) => "JumpingReturn",
            DogStateMachine::JumpingWorried(_) => "JumpingWorried",
            DogStateMachine::JumpingWorriedReturn(_) => "JumpingWorriedReturn",
            DogStateMachine::Returning(_) => "Returning",
            DogStateMachine::ReturningToFlee(_) => "ReturningToFlee",
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
            (DogStateMachine::Fleeing(state), Event::Jump) => state.jump().into(),
            (DogStateMachine::Fleeing(state), Event::OffPlatform) => {
                state.drop_from_platform().into()
            }
            (DogStateMachine::Fleeing(state), Event::Update) => state.update().into(),
            (DogStateMachine::Fleeing(state), Event::Worry) => state.worry().into(),

            (DogStateMachine::Jumping(state), Event::Land(p)) => state.land_on(p).into(),
            (DogStateMachine::Jumping(state), Event::Update) => state.update().into(),

            (DogStateMachine::JumpingFlee(state), Event::Land(p)) => state.land_on(p).into(),
            (DogStateMachine::JumpingFlee(state), Event::Update) => state.update().into(),

            (DogStateMachine::JumpingFleeReturn(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::JumpingFleeReturn(state), Event::Update) => state.update().into(),

            (DogStateMachine::JumpingReturn(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::JumpingReturn(state), Event::Update) => state.update().into(),

            (DogStateMachine::JumpingWorried(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::JumpingWorried(state), Event::Update) => state.update().into(),

            (DogStateMachine::JumpingWorriedReturn(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::JumpingWorriedReturn(state), Event::Update) => state.update().into(),

            (DogStateMachine::Returning(state), Event::Flee) => state.flee().into(),
            (DogStateMachine::Returning(state), Event::Jump) => state.jump().into(),
            (DogStateMachine::Returning(state), Event::Update) => state.update().into(),

            (DogStateMachine::ReturningToFlee(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::ReturningToFlee(state), Event::Jump) => state.jump().into(),
            (DogStateMachine::ReturningToFlee(state), Event::Update) => state.update().into(),
            (DogStateMachine::ReturningToFlee(state), Event::Worry) => state.worry().into(),

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

impl From<DogState<Fleeing>> for DogStateMachine {
    fn from(state: DogState<Fleeing>) -> Self {
        DogStateMachine::Fleeing(state)
    }
}

impl From<DogState<Jumping>> for DogStateMachine {
    fn from(state: DogState<Jumping>) -> Self {
        DogStateMachine::Jumping(state)
    }
}

impl From<DogState<JumpingFlee>> for DogStateMachine {
    fn from(state: DogState<JumpingFlee>) -> Self {
        DogStateMachine::JumpingFlee(state)
    }
}

impl From<DogState<JumpingFleeReturn>> for DogStateMachine {
    fn from(state: DogState<JumpingFleeReturn>) -> Self {
        DogStateMachine::JumpingFleeReturn(state)
    }
}

impl From<DogState<JumpingReturn>> for DogStateMachine {
    fn from(state: DogState<JumpingReturn>) -> Self {
        DogStateMachine::JumpingReturn(state)
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
