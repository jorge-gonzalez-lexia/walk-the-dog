use super::{
    context::DogContext,
    states::{returning::Returning, running::Running, DogState},
};

#[derive(Clone)]
pub enum DogStateMachine {
    Returning(DogState<Returning>),
    Running(DogState<Running>),
}

impl DogStateMachine {
    pub fn context(&self) -> &DogContext {
        match self {
            DogStateMachine::Returning(state) => state.context(),
            DogStateMachine::Running(state) => state.context(),
        }
    }

    pub fn update(self) -> Self {
        match self {
            DogStateMachine::Returning(state) => state.update().into(),
            DogStateMachine::Running(state) => state.update().into(),
        }
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
