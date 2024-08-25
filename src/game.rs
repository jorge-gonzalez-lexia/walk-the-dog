mod red_hat_boy_states;

use self::red_hat_boy_states::*;
use crate::{
    browser,
    engine::{self, Game, KeyState, Rect, Renderer},
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
    rhb: Option<RedHatBoy>,
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog { rhb: None }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });
        self.rhb.as_ref().unwrap().draw(renderer);
    }

    async fn initialize(&self) -> Result<Box<dyn Game>> {
        // TODO: into_serde is deprecated (presumably after book was written)
        let sheet: Option<Sheet> = browser::fetch_json("rhb.json").await?.into_serde()?;
        let image = Some(engine::load_image("rhb.png").await?);

        Ok(Box::new(WalkTheDog {
            rhb: Some(RedHatBoy::new(
                sheet.clone().ok_or_else(|| anyhow!("No Sheet Present"))?,
                image.clone().ok_or_else(|| anyhow!("No Image Present"))?,
            )),
        }))
    }

    fn update(&mut self, keystate: &KeyState) {
        if keystate.is_pressed("ArrowRight") {
            self.rhb.as_mut().unwrap().run_right();
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

    fn run_right(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Run);
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
                state.update();

                RedHatBoyStateMachine::Idle(state)
            }
            RedHatBoyStateMachine::Running(mut state) => {
                state.update();

                RedHatBoyStateMachine::Running(state)
            }
        }
    }
}

impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Running>) -> Self {
        RedHatBoyStateMachine::Running(state)
    }
}
