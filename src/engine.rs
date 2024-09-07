pub mod audio;
pub mod image;
pub mod input;
pub mod rect;
pub mod renderer;
pub mod sheet;
pub mod sprite_sheet;

use crate::browser::{self, LoopClosure};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use input::KeyState;
use rect::Point;
use renderer::Renderer;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[async_trait(?Send)]
pub trait Game {
    fn draw(&self, context: &Renderer);

    async fn initialize(&self) -> Result<Box<dyn Game>>;

    fn update(&mut self, keystate: &KeyState);
}

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;

pub struct GameLoop {
    accumulated_delta: f32,
    last_frame: f64,
}

type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()> {
        let mut keyevent_receiver = input::prepare_input()?;
        let mut game = game.initialize().await?;
        let mut game_loop = GameLoop {
            last_frame: browser::now()?,
            accumulated_delta: 0.0,
        };
        let renderer = Renderer {
            context: browser::context()?,
        };

        // The use of Rc and RefCell is described in book pages 140-145. It is what
        // allows borrowing the closure multiple times.
        let f: SharedLoopClosure = Rc::new(RefCell::new(None));

        let mut keystate = KeyState::new();
        let g = f.clone();
        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            input::process_input(&mut keystate, &mut keyevent_receiver);
            // Fixing the time step. See pp 145-151
            let frame_time = perf - game_loop.last_frame;
            game_loop.accumulated_delta += frame_time as f32;
            while game_loop.accumulated_delta > FRAME_SIZE {
                game.update(&keystate);
                game_loop.accumulated_delta -= FRAME_SIZE;
            }
            game_loop.last_frame = perf;

            game.draw(&renderer);
            if cfg!(debug_assertions) {
                unsafe { draw_frame_rate(&renderer, frame_time) }
            }

            browser::request_animation_frame(f.borrow().as_ref().unwrap()).unwrap();
        }));

        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
        )?;

        Ok(())
    }
}

pub fn add_click_handler(elem: HtmlElement) -> UnboundedReceiver<()> {
    let (mut click_sender, click_receiver) = unbounded();
    let on_click = browser::closure_wrap(Box::new(move || {
        if let Err(err) = click_sender.start_send(()) {
            error!("Error sending click event ${:#?}", err);
        }
    }) as Box<dyn FnMut()>);
    elem.set_onclick(Some(on_click.as_ref().unchecked_ref()));
    on_click.forget();

    click_receiver
}

// fn is unsafe because of static mut (not thread safe), which is ok
unsafe fn draw_frame_rate(renderer: &Renderer, frame_time: f64) {
    static mut FRAMES_COUNTED: i32 = 0;
    static mut TOTAL_FRAME_TIME: f64 = 0.0;
    static mut FRAME_RATE: i32 = 0;

    FRAMES_COUNTED += 1;
    TOTAL_FRAME_TIME += frame_time;

    if TOTAL_FRAME_TIME > 1000.0 {
        FRAME_RATE = FRAMES_COUNTED;
        TOTAL_FRAME_TIME = 0.0;
        FRAMES_COUNTED = 0;
    }

    if let Err(err) = renderer.draw_text(
        &format!("Frame Rate {}", FRAME_RATE),
        &Point { x: 400, y: 100 },
    ) {
        error!("Could not draw text {:#?}", err);
    }
}
