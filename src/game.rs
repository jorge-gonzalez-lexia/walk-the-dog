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

pub enum WalkTheDog {
    Loaded(RedHatBoy),
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
        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });

        if let WalkTheDog::Loaded(rhb) = self {
            rhb.draw(renderer);
        }
    }

    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self {
            WalkTheDog::Loading => {
                let json = browser::fetch_json("rhb.json").await?;
                let rhb = RedHatBoy::new(
                    // TODO: into_serde is deprecated (presumably after book was written)
                    json.into_serde::<Sheet>()?,
                    engine::load_image("rhb.png").await?,
                );

                Ok(Box::new(WalkTheDog::Loaded(rhb)))
            }

            WalkTheDog::Loaded(_) => Err(anyhow!("Error: Game is already initialized!")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
        if let WalkTheDog::Loaded(rhb) = self {
            if keystate.is_pressed("ArrowRight") {
                rhb.run_right();
            }
            if keystate.is_pressed("ArrowDown") {
                rhb.slide();
            }
            if keystate.is_pressed("Space") {
                rhb.jump();
            }

            rhb.update();
        }
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

    fn jump(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Jump);
    }

    fn run_right(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Run);
    }

    fn slide(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Slide);
    }

    fn update(&mut self) {
        self.state_machine = self.state_machine.update();
    }
}

// See p214. This could be implemented as a trait object instead
#[derive(Clone, Copy)]
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Jumping(RedHatBoyState<Jumping>),
    Running(RedHatBoyState<Running>),
    Sliding(RedHatBoyState<Sliding>),
}

pub enum Event {
    Jump,
    Run,
    Slide,
    Update,
}

impl RedHatBoyStateMachine {
    fn context(&self) -> &RedHatBoyContext {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.context(),
            RedHatBoyStateMachine::Jumping(state) => &state.context(),
            RedHatBoyStateMachine::Running(state) => &state.context(),
            RedHatBoyStateMachine::Sliding(state) => &state.context(),
        }
    }

    fn frame_name(&self) -> &str {
        match self {
            RedHatBoyStateMachine::Idle(state) => state.frame_name(),
            RedHatBoyStateMachine::Jumping(state) => state.frame_name(),
            RedHatBoyStateMachine::Running(state) => state.frame_name(),
            RedHatBoyStateMachine::Sliding(state) => state.frame_name(),
        }
    }

    fn transition(self, event: Event) -> Self {
        match (self, event) {
            (RedHatBoyStateMachine::Running(state), Event::Jump) => state.jump().into(),
            (RedHatBoyStateMachine::Idle(state), Event::Run) => state.run().into(),
            (RedHatBoyStateMachine::Running(state), Event::Slide) => state.slide().into(),

            (RedHatBoyStateMachine::Idle(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Jumping(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Running(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Sliding(state), Event::Update) => state.update().into(),
            _ => self,
        }
    }

    fn update(self) -> Self {
        self.transition(Event::Update)
    }
}

impl From<RedHatBoyState<Idle>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Idle>) -> Self {
        RedHatBoyStateMachine::Idle(state)
    }
}

impl From<RedHatBoyState<Jumping>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Jumping>) -> Self {
        RedHatBoyStateMachine::Jumping(state)
    }
}

impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Running>) -> Self {
        RedHatBoyStateMachine::Running(state)
    }
}

impl From<RedHatBoyState<Sliding>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Sliding>) -> Self {
        RedHatBoyStateMachine::Sliding(state)
    }
}
