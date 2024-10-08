use super::{game_over::GameOver, WalkTheDogState, WalkTheDogStateMachine};
use crate::{
    browser,
    engine::{self, input::KeyState},
};

pub struct Walking;

const TIMELINE_MINIMUM: i16 = 1000;

impl WalkTheDogState<Walking> {
    pub fn update(mut self, keystate: &KeyState) -> WalkingEndState {
        if keystate.is_pressed("Space") {
            self.walk.boy.jump();
        }
        if keystate.is_pressed("ArrowDown") {
            self.walk.boy.slide();
        }

        self.walk.boy.update();
        let walking_speed = self.walk.velocity();

        let [first_background, second_background] = &mut self.walk.backgrounds;
        first_background.move_horizontally(walking_speed);
        second_background.move_horizontally(walking_speed);
        if first_background.right() < 0 {
            first_background.set_x(second_background.right());
        }
        if second_background.right() < 0 {
            second_background.set_x(first_background.right());
        }

        self.walk.drop_surpassed_obstacles();

        for obstacle in self.walk.obstacles.iter() {
            obstacle.borrow_mut().move_horizontally(walking_speed);
            obstacle.borrow_mut().check_intersection(&mut self.walk.boy);
        }
        self.walk.update();

        if self.walk.timeline < TIMELINE_MINIMUM {
            self.walk.generate_next_segment();
        } else {
            self.walk.timeline += walking_speed;
        }

        if self.walk.knocked_out() {
            WalkingEndState::Complete(self.end_game())
        } else {
            WalkingEndState::Continue(self)
        }
    }

    fn end_game(self) -> WalkTheDogState<GameOver> {
        let new_game_event = browser::draw_ui("<button id='new_game'>New Game</button>")
            .and_then(|_| browser::find_html_element_by_id("new_game"))
            .map(engine::add_click_handler)
            .unwrap();

        WalkTheDogState {
            walk: self.walk,
            _state: GameOver { new_game_event },
        }
    }
}

pub enum WalkingEndState {
    Complete(WalkTheDogState<GameOver>),
    Continue(WalkTheDogState<Walking>),
}

impl From<WalkingEndState> for WalkTheDogStateMachine {
    fn from(state: WalkingEndState) -> Self {
        match state {
            WalkingEndState::Complete(game_over) => game_over.into(),
            WalkingEndState::Continue(walking) => walking.into(),
        }
    }
}
