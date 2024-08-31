mod platform;
mod red_hat_boy;
mod walk;

use crate::{
    browser,
    engine::{
        image::{load_image, Image},
        input::KeyState,
        rect::{Point, Rect},
        renderer::Renderer,
        sheet::Sheet,
        Game,
    },
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use platform::Platform;
use red_hat_boy::RedHatBoy;
use walk::Walk;

const HEIGHT: i16 = 600;

const FIRST_PLATFORM: i16 = 270;
const HIGH_PLATFORM: i16 = 375;
const LOW_PLATFORM: i16 = 420;

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
            walk.platform.draw(renderer);
            walk.stone.draw(renderer);
        }
    }

    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self {
            WalkTheDog::Loading => {
                let json = browser::fetch_json("rhb.json").await?;
                let rhb = RedHatBoy::new(
                    // TODO: into_serde is deprecated (presumably after book was written)
                    json.into_serde::<Sheet>()?,
                    load_image("rhb.png").await?,
                );

                let background = load_image("BG.png").await?;
                let stone = load_image("Stone.png").await?;

                let platform_sheet = browser::fetch_json("tiles.json").await?;
                let platform = Platform::new(
                    platform_sheet.into_serde::<Sheet>()?,
                    load_image("tiles.png").await?,
                    Point {
                        x: FIRST_PLATFORM,
                        y: HIGH_PLATFORM,
                    },
                );

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
                    platform,
                    stone: Image::new(stone, Point { x: 150, y: 546 }),
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
            walk.platform.position.x += walk.velocity();
            walk.stone.move_horizontally(walk.velocity());
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

            for bounding_box in &walk.platform.bounding_boxes() {
                if walk.boy.bounding_box().intersects(bounding_box) {
                    if walk.boy.velocity_y() > 0 && walk.boy.position_y() < walk.platform.position.y
                    {
                        walk.boy.land_on(walk.platform.destination_box().top());
                    } else {
                        walk.boy.knock_out();
                    }
                }
            }

            if walk
                .boy
                .bounding_box()
                .intersects(walk.stone.bounding_box())
            {
                walk.boy.knock_out();
            }
        }
    }
}
