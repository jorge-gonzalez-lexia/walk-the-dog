use super::{
    context::DogContext,
    states::{
        fleeing::Fleeing, jumping::Jumping, jumping_flee::JumpingFlee,
        jumping_flee_return::JumpingFleeReturn, jumping_return::JumpingReturn,
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
            (DogStateMachine::Fleeing(state), Event::Jump) => state.jump().into(),
            (DogStateMachine::Fleeing(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::Fleeing(state), Event::Update) => state.update().into(),
            (DogStateMachine::Fleeing(state), Event::Worry) => state.worry().into(),

            (DogStateMachine::Jumping(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::Jumping(state), Event::Update) => state.update().into(),

            (DogStateMachine::JumpingFlee(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::JumpingFlee(state), Event::Update) => state.update().into(),

            (DogStateMachine::JumpingFleeReturn(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::JumpingFleeReturn(state), Event::Update) => state.update().into(),

            (DogStateMachine::JumpingReturn(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::JumpingReturn(state), Event::Update) => state.update().into(),

            (DogStateMachine::Returning(state), Event::Flee) => state.flee().into(),
            (DogStateMachine::Returning(state), Event::Jump) => state.jump().into(),
            (DogStateMachine::Returning(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::Returning(state), Event::Update) => state.update().into(),

            (DogStateMachine::ReturningToFlee(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (DogStateMachine::ReturningToFlee(state), Event::Jump) => state.jump().into(),
            (DogStateMachine::ReturningToFlee(state), Event::Update) => state.update().into(),
            (DogStateMachine::ReturningToFlee(state), Event::Worry) => state.worry().into(),

            (DogStateMachine::Running(state), Event::Flee) => state.flee().into(),
            (DogStateMachine::Running(state), Event::Jump) => state.jump().into(),
            (DogStateMachine::Running(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
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
