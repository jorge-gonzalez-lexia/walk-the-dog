use super::WalkTheDogState;

pub struct GameOver;

impl WalkTheDogState<GameOver> {
    pub fn update(self) -> WalkTheDogState<GameOver> {
        self
    }
}
