mod red_hat_boy_states;

use self::red_hat_boy_states::*;
use crate::{
    browser,
    engine::{self, Game, KeyState, Point, Rect, Renderer},
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use web_sys::HtmlImageElement;

#[derive(Clone, Deserialize)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}

pub struct WalkTheDog {
    frame: u8,
    image: Option<HtmlImageElement>,
    position: Point,
    rhb: Option<RedHatBoy>,
    sheet: Option<Sheet>,
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog {
            frame: 0,
            image: None,
            position: Point { x: 0, y: 0 },
            rhb: None,
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

        self.rhb.as_ref().unwrap().draw(renderer);
    }

    async fn initialize(&self) -> Result<Box<dyn Game>> {
        // TODO: into_serde is deprecated (presumably after book was written)
        let sheet: Option<Sheet> = browser::fetch_json("rhb.json").await?.into_serde()?;
        let image = Some(engine::load_image("rhb.png").await?);

        Ok(Box::new(WalkTheDog {
            image: image.clone(),
            frame: self.frame,
            position: self.position,
            rhb: Some(RedHatBoy::new(
                sheet.clone().ok_or_else(|| anyhow!("No Sheet Present"))?,
                image.clone().ok_or_else(|| anyhow!("No Image Present"))?,
            )),
            sheet: sheet.clone(),
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

        self.rhb.as_mut().unwrap().update();
    }
}

#[derive(Clone, Deserialize)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Clone, Deserialize)]
struct Cell {
    frame: SheetRect,
}

struct RedHatBoy {
    image: HtmlImageElement,
    sprite_sheet: Sheet,
    state_machine: RedHatBoyStateMachine,
}

impl RedHatBoy {
    pub fn new(sprite_sheet: Sheet, image: HtmlImageElement) -> Self {
        RedHatBoy {
            image,
            sprite_sheet,
            state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::new()),
        }
    }

    fn draw(&self, renderer: &Renderer) {
        let frame_name = format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            (self.state_machine.context().frame / 3) + 1
        );
        let sprite = self
            .sprite_sheet
            .frames
            .get(&frame_name)
            .expect("Sprite sheet cell not found");
        renderer.draw_image(
            &self.image,
            &Rect {
                x: sprite.frame.x.into(),
                y: sprite.frame.y.into(),
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            },
            &Rect {
                x: self.state_machine.context().position.x.into(),
                y: self.state_machine.context().position.y.into(),
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            },
        );
    }

    fn update(&mut self) {
        self.state_machine = self.state_machine.update();
    }
}

// See p214. This could be implemented as a trait object instead
#[derive(Clone, Copy)]
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
}

pub enum Event {
    Run,
}

impl RedHatBoyStateMachine {
    fn context(&self) -> &RedHatBoyContext {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.context(),
            RedHatBoyStateMachine::Running(state) => &state.context(),
        }
    }

    fn frame_name(&self) -> &str {
        match self {
            RedHatBoyStateMachine::Idle(state) => state.frame_name(),
            RedHatBoyStateMachine::Running(state) => state.frame_name(),
        }
    }

    fn transition(self, event: Event) -> Self {
        match (self, event) {
            (RedHatBoyStateMachine::Idle(state), Event::Run) => state.run().into(),
            _ => self,
        }
    }

    fn update(self) -> Self {
        match self {
            RedHatBoyStateMachine::Idle(mut state) => {
                if state.context.frame < 29 {
                    state.context.frame += 1;
                } else {
                    state.context.frame = 0;
                }

                RedHatBoyStateMachine::Idle(state)
            }
            RedHatBoyStateMachine::Running(_) => self,
            // RedHatBoyStateMachine::Running(mut state) => {
            //     if state.context.frame < 23 {
            //         state.context.frame += 1;
            //     } else {
            //         state.context.frame = 0;
            //     }

            //     RedHatBoyStateMachine::Running(state)
            // }
        }
    }
}

impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Running>) -> Self {
        RedHatBoyStateMachine::Running(state)
    }
}
