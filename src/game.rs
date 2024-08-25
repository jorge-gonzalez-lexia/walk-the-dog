mod red_hat_boy_states;

use self::red_hat_boy_states::*;
use crate::{
    browser,
    engine::{self, Game, KeyState, Point, Rect, Renderer},
};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use web_sys::HtmlImageElement;

#[derive(Deserialize)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}

pub struct WalkTheDog {
    frame: u8,
    image: Option<HtmlImageElement>,
    position: Point,
    sheet: Option<Sheet>,
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog {
            frame: 0,
            image: None,
            position: Point { x: 0, y: 0 },
            sheet: None,
        }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    fn draw(&self, renderer: &Renderer) {
        // self.frame = 0-22. convert to sprite animation frame = 1-8
        let current_sprite = (self.frame / 3) + 1;
        let frame_name = format!("Run ({}).png", current_sprite);
        let sprite = self
            .sheet
            .as_ref()
            .and_then(|sheet| sheet.frames.get(&frame_name))
            .expect("Cell not found");
        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });
        self.image.as_ref().map(|image| {
            renderer.draw_image(
                &image,
                &Rect {
                    x: sprite.frame.x.into(),
                    y: sprite.frame.y.into(),
                    width: sprite.frame.w.into(),
                    height: sprite.frame.h.into(),
                },
                &Rect {
                    x: self.position.x.into(),
                    y: self.position.y.into(),
                    width: sprite.frame.w.into(),
                    height: sprite.frame.h.into(),
                },
            );
        });
    }

    async fn initialize(&self) -> Result<Box<dyn Game>> {
        // TODO: into_serde is deprecated (presumably after book was written)
        let sheet = browser::fetch_json("rhb.json").await?.into_serde()?;
        let image = Some(engine::load_image("rhb.png").await?);

        Ok(Box::new(WalkTheDog {
            image,
            frame: self.frame,
            position: self.position,
            sheet,
        }))
    }

    fn update(&mut self, keystate: &KeyState) {
        let mut velocity = Point { x: 0, y: 0 };
        if keystate.is_pressed("ArrowDown") {
            velocity.y += 3;
        }
        if keystate.is_pressed("ArrowUp") {
            velocity.y -= 3;
        }
        if keystate.is_pressed("ArrowRight") {
            velocity.x += 3;
        }
        if keystate.is_pressed("ArrowLeft") {
            velocity.x -= 3;
        }

        self.position.x += velocity.x;
        self.position.y += velocity.y;

        if velocity.x != 0 || velocity.y != 0 {
            log!(
                "velocity ({},{}) position ({},{})",
                velocity.x,
                velocity.y,
                self.position.x,
                self.position.y
            );
        }

        // See p. 162. Given running animation has 8 frames, it should change
        // every 3 updates to change animation frame every ~50ms
        // (16.7ms/frame * 3 ~= 50ms)
        if self.frame < 23 {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
    }
}

#[derive(Deserialize)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Deserialize)]
struct Cell {
    frame: SheetRect,
}

#[derive(Clone, Copy)]
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
}

pub enum Event {
    Run,
}

impl RedHatBoyStateMachine {
    fn transition(self, event: Event) -> Self {
        match (self, event) {
            (RedHatBoyStateMachine::Idle(state), Event::Run) => {
                RedHatBoyStateMachine::Running(state.run())
            }
            _ => self,
        }
    }
}
