use super::{ready::Ready, WalkTheDogState, WalkTheDogStateMachine};
use crate::{browser, game::walk::Walk};
use futures::channel::mpsc::UnboundedReceiver;

pub struct GameOver {
    pub new_game_event: UnboundedReceiver<()>,
}

impl GameOver {
    pub fn new_game_pressed(&mut self) -> bool {
        matches!(self.new_game_event.try_next(), Ok(Some(())))
    }
}

impl WalkTheDogState<GameOver> {
    pub fn update(mut self) -> GameOverEndState {
        if self._state.new_game_pressed() {
            GameOverEndState::Complete(self.new_game())
        } else {
            self.walk.process_events();

            self.walk.dog.update();
            self.walk.obstacles.iter_mut().for_each(|obstacle| {
                obstacle.navigate(&self.walk.dog);
            });
            GameOverEndState::Continue(self)
        }
    }

    fn new_game(self) -> WalkTheDogState<Ready> {
        if let Err(err) = browser::hide_ui() {
            error!("Error hiding the UI overlay {:#?}", err);
        }

        WalkTheDogState {
            walk: Walk::reset(self.walk),
            _state: Ready,
        }
    }
}

pub enum GameOverEndState {
    Complete(WalkTheDogState<Ready>),
    Continue(WalkTheDogState<GameOver>),
}

impl From<GameOverEndState> for WalkTheDogStateMachine {
    fn from(state: GameOverEndState) -> Self {
        match state {
            GameOverEndState::Complete(ready) => ready.into(),
            GameOverEndState::Continue(game_over) => game_over.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        engine::{
            audio::{Audio, Sound},
            image::Image,
            rect::Point,
            sheet::Sheet,
            sprite_sheet::SpriteSheet,
        },
        game::{
            dog::Dog,
            event_queue::EventPublisher,
            red_hat_boy::{context::Sfx, RedHatBoy},
            segments::SegmentFactory,
        },
    };
    use futures::channel::mpsc::unbounded;
    use std::{
        cell::RefCell,
        collections::{HashMap, VecDeque},
        rc::Rc,
    };
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::{AudioBuffer, AudioBufferOptions, HtmlImageElement};
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_transition_from_game_over_to_new_game() {
        let (_, receiver) = unbounded();
        let image = HtmlImageElement::new().unwrap();
        let audio = Audio::new().unwrap();
        let options = AudioBufferOptions::new(1, 3000.0);
        let sound = Sound {
            buffer: AudioBuffer::new(&options).unwrap(),
        };
        let sfx = Sfx::new(sound.clone(), sound.clone(), sound.clone());
        let events = Rc::new(RefCell::new(VecDeque::new()));
        let event_publisher = EventPublisher::new(events.clone());

        let boy = RedHatBoy::new(
            audio,
            sfx,
            Sheet {
                frames: HashMap::new(),
            },
            image.clone(),
        );
        let dog = Dog::new(
            Sheet {
                frames: HashMap::new(),
            },
            image.clone(),
            event_publisher.clone(),
        );
        let sprite_sheet = Rc::new(SpriteSheet::new(
            Sheet {
                frames: HashMap::new(),
            },
            image.clone(),
        ));
        let segment_factory = SegmentFactory::new(sprite_sheet.clone(), image.clone());
        let walk = Walk {
            backgrounds: [
                Image::new(image.clone(), Point { x: 0, y: 0 }),
                Image::new(image.clone(), Point { x: 0, y: 0 }),
            ],
            boy,
            dog,
            event_publisher,
            events,
            obstacles: vec![],
            segment_factory,
            stone: image.clone(),
            timeline: 9,
        };

        let document = browser::document().unwrap();
        document
            .body()
            .unwrap()
            .insert_adjacent_html("afterbegin", "<div id='ui'></div>")
            .unwrap();
        browser::draw_ui("<p>This is the UI</p>").unwrap();

        let state = WalkTheDogState {
            walk,
            _state: GameOver {
                new_game_event: receiver,
            },
        };

        state.new_game();

        let ui = browser::find_html_element_by_id("ui").unwrap();
        assert_eq!(ui.child_element_count(), 0);
    }
}
