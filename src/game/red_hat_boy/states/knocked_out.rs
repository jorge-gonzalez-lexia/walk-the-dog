use super::RedHatBoyState;

const DEAD_FRAME_NAME: &str = "Dead";

#[derive(Clone, Copy)]
pub struct KnockedOut;

impl RedHatBoyState<KnockedOut> {
    pub fn frame_name(&self) -> &str {
        DEAD_FRAME_NAME
    }
}
