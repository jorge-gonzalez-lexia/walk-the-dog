pub mod falling;
pub mod idle;
pub mod jumping;
pub mod knocked_out;
pub mod running;
pub mod sliding;

use super::context::RedHatBoyContext;

#[derive(Clone)]
pub struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

impl<S> RedHatBoyState<S> {
    pub fn context(&self) -> &RedHatBoyContext {
        &self.context
    }
}
