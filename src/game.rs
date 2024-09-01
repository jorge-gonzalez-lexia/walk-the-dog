mod barrier;
mod obstacle;
mod platform;
mod red_hat_boy;
mod segments;
mod walk;

use crate::{
    browser,
    engine::{
        audio::{self, Audio},
        image::{load_image, Image},
        input::KeyState,
        rect::{Point, Rect},
        renderer::Renderer,
        sheet::Sheet,
        sprite_sheet::SpriteSheet,
        Game,
    },
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::TryFutureExt;
use red_hat_boy::RedHatBoy;
use segments::stone_and_platform;
use std::rc::Rc;
use walk::{rightmost, Walk};

const HEIGHT: i16 = 600;
const TIMELINE_MINIMUM: i16 = 1000;

pub enum WalkTheDog {
    Loaded(Walk),
    Loading,
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog::Loading
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect::new_from_x_y(0, 0, 600, HEIGHT));

        if let WalkTheDog::Loaded(walk) = self {
            walk.backgrounds.iter().for_each(|b| b.draw(renderer));
            walk.boy.draw(renderer);
            walk.obstacles.iter().for_each(|obstacle| {
                obstacle.draw(renderer);
            });
        }
    }

    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self {
            WalkTheDog::Loading => {
                let audio = Audio::new()?;
                let sound = audio.load_sound("SFX_Jump_23.mp3").await?;
                let json = browser::fetch_json("rhb.json").await?;
                let rhb = RedHatBoy::new(
                    audio,
                    sound,
                    // TODO: into_serde is deprecated (presumably after book was written)
                    json.into_serde::<Sheet>()?,
                    load_image("rhb.png").await?,
                );

                let background = load_image("BG.png").await?;
                let stone = load_image("Stone.png").await?;

                let tiles = browser::fetch_json("tiles.json").await?;
                let sprite_sheet = Rc::new(SpriteSheet::new(
                    tiles.into_serde::<Sheet>()?,
                    load_image("tiles.png").await?,
                ));

                let starting_obstacles = stone_and_platform(stone.clone(), sprite_sheet.clone(), 0);
                let timeline = rightmost(&starting_obstacles);

                let background_width = background.width() as i16;

                Ok(Box::new(WalkTheDog::Loaded(Walk {
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
                    boy: rhb,
                    obstacle_sheet: sprite_sheet,
                    obstacles: starting_obstacles,
                    stone,
                    timeline,
                })))
            }

            WalkTheDog::Loaded(_) => Err(anyhow!("Error: Game is already initialized!")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
        if let WalkTheDog::Loaded(walk) = self {
            if keystate.is_pressed("ArrowRight") {
                walk.boy.run_right();
            }
            if keystate.is_pressed("ArrowDown") {
                walk.boy.slide();
            }
            if keystate.is_pressed("Space") {
                walk.boy.jump();
            }

            walk.boy.update();
            let velocity = walk.velocity();

            let [first_background, second_background] = &mut walk.backgrounds;
            first_background.move_horizontally(velocity);
            second_background.move_horizontally(velocity);
            if first_background.right() < 0 {
                first_background.set_x(second_background.right());
            }
            if second_background.right() < 0 {
                second_background.set_x(first_background.right());
            }

            walk.obstacles.retain(|obstacle| obstacle.right() > 0);

            walk.obstacles.iter_mut().for_each(|obstacle| {
                obstacle.move_horizontally(velocity);
                obstacle.check_intersection(&mut walk.boy)
            });

            if walk.timeline < TIMELINE_MINIMUM {
                walk.generate_next_segment();
            } else {
                walk.timeline += velocity;
            }
        }
    }
}
