use crate::engine::Point;

#[derive(Clone, Copy)]
pub struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

#[derive(Clone, Copy)]
pub struct RedHatBoyContext {
    frame: u8,
    position: Point,
    velocity: Point,
}

#[derive(Clone, Copy)]
pub struct Idle;

#[derive(Clone, Copy)]
pub struct Running;
