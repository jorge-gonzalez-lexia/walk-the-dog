use super::{
    context::DogContext,
    states::{running::Running, DogState},
};

pub enum DogStateMachine {
    Running(DogState<Running>),
}

impl DogStateMachine {
    pub fn context(&self) -> &DogContext {
        match self {
            DogStateMachine::Running(state) => state.context(),
        }
    }
}
