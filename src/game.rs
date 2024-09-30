mod barrier;
mod dog;
mod event_queue;
pub mod game_states;
mod obstacle;
mod platform;
mod red_hat_boy;
mod segments;
mod walk;

use crate::engine::{
    audio::Audio,
    image::{load_image, Image},
    input::KeyState,
    rect::{Point, Rect},
    renderer::Renderer,
    sheet::Sheet,
    sprite_sheet::SpriteSheet,
    Game,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use dog::Dog;
use event_queue::EventPublisher;
use game_states::WalkTheDogStateMachine;
use red_hat_boy::{context::Sfx, RedHatBoy};
use segments::SegmentFactory;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
use walk::{rightmost, Walk};

const GRAVITY: i16 = 1;
const TERMINAL_VELOCITY: i16 = 20;

const HEIGHT: i16 = 600;

pub struct WalkTheDog {
    machine: Option<WalkTheDogStateMachine>,
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog { machine: None }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect::new_from_x_y(0, 0, 600, HEIGHT));

        if let Some(machine) = &self.machine {
            machine.draw(renderer);
        }
    }

    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self.machine {
            None => {
                let audio = Audio::new()?;
                let sfx = Sfx::new(
                    audio.load_sound("SFX_Jump_23.mp3").await?,
                    audio.load_sound("vgdeathsound.ogg").await?,
                    audio.load_sound("slide.wav").await?,
                );
                let background_music = audio.load_sound("background_song.mp3").await?;

                // audio.play_looping_sound(&background_music)?;

                let events = Rc::new(RefCell::new(VecDeque::new()));
                let event_publisher = EventPublisher::new(events.clone());

                let boy = RedHatBoy::new(
                    audio,
                    sfx,
                    Sheet::load("rhb.json").await?,
                    load_image("rhb.png").await?,
                );
                let dog = Dog::new(
                    Sheet::load("dog.json").await?,
                    load_image("dog.png").await?,
                    event_publisher.clone(),
                );

                let background = load_image("BG.png").await?;
                let stone = load_image("Stone.png").await?;

                let sprite_sheet = Rc::new(SpriteSheet::new(
                    Sheet::load("tiles.json").await?,
                    load_image("tiles.png").await?,
                ));

                let mut segment_factory = SegmentFactory::new(
                    sprite_sheet.clone(),
                    stone.clone(),
                    event_publisher.clone(),
                );

                let starting_obstacles = segment_factory.first();
                let timeline = rightmost(&starting_obstacles);

                let background_width = background.width() as i16;

                let machine = WalkTheDogStateMachine::new(Walk {
                    backgrounds: [
                        Image::new(background.clone(), Point { x: 0, y: 0 }),
                        Image::new(
                            background,
                            Point {
                                x: background_width,
                                y: 0,
                            },
                        ),
                    ],
                    boy,
                    dog,
                    events,
                    event_publisher,
                    obstacles: starting_obstacles,
                    segment_factory,
                    stone,
                    timeline,
                });

                Ok(Box::new(WalkTheDog {
                    machine: Some(machine),
                }))
            }

            Some(_) => Err(anyhow!("Error: Game is already initialized!")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
        if let Some(machine) = self.machine.take() {
            self.machine.replace(machine.update(keystate));
        }

        assert!(self.machine.is_some());
    }
}
