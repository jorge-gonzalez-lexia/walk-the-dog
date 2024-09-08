use super::{
    context::DogContext,
    states::{running::Running, DogState},
};

#[derive(Clone)]
pub enum DogStateMachine {
    Running(DogState<Running>),
}

impl DogStateMachine {
    pub fn context(&self) -> &DogContext {
        match self {
            DogStateMachine::Running(state) => state.context(),
        }
    }

    pub fn update(self) -> Self {
        match self {
            DogStateMachine::Running(state) => state.update().into(),
        }
    }
}

impl From<DogState<Running>> for DogStateMachine {
    fn from(state: DogState<Running>) -> Self {
        DogStateMachine::Running(state)
    }
}
