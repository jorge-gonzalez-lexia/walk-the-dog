use super::WalkTheDogState;
use crate::engine::input::KeyState;

pub struct Walking;

const TIMELINE_MINIMUM: i16 = 1000;

impl WalkTheDogState<Walking> {
    pub fn update(mut self, keystate: &KeyState) -> WalkTheDogState<Walking> {
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

        self.walk.obstacles.retain(|obstacle| obstacle.right() > 0);

        self.walk.obstacles.iter_mut().for_each(|obstacle| {
            obstacle.move_horizontally(walking_speed);
            obstacle.check_intersection(&mut self.walk.boy)
        });

        if self.walk.timeline < TIMELINE_MINIMUM {
            self.walk.generate_next_segment();
        } else {
            self.walk.timeline += walking_speed;
        }

        self
    }
}
