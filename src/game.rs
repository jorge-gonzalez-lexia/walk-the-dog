mod red_hat_boy_states;

use self::red_hat_boy_states::*;
use crate::{
    browser,
    engine::{self, Cell, Game, Image, KeyState, Point, Rect, Renderer, Sheet},
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use web_sys::HtmlImageElement;

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

pub struct Walk {
    backgrounds: [Image; 2],
    boy: RedHatBoy,
    platform: Platform,
    stone: Image,
}

impl Walk {
    fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect {
            x: 0,
            y: 0,
            width: 600,
            height: HEIGHT,
        });

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
                    engine::load_image("rhb.png").await?,
                );

                let background = engine::load_image("BG.png").await?;
                let stone = engine::load_image("Stone.png").await?;

                let platform_sheet = browser::fetch_json("tiles.json").await?;
                let platform = Platform::new(
                    platform_sheet.into_serde::<Sheet>()?,
                    engine::load_image("tiles.png").await?,
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
                        walk.boy.land_on(walk.platform.destination_box().y);
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

    fn bounding_box(&self) -> Rect {
        const X_OFFSET: i16 = 18;
        const Y_OFFSET: i16 = 14;
        const WIDTH_OFFSET: i16 = 28;

        let mut bounding_box = self.destination_box();
        bounding_box.x += X_OFFSET;
        bounding_box.width -= WIDTH_OFFSET;
        bounding_box.y += Y_OFFSET;
        bounding_box.height -= Y_OFFSET;

        bounding_box
    }

    fn current_sprite(&self) -> Option<&Cell> {
        self.sprite_sheet.frames.get(&self.frame_name())
    }

    fn destination_box(&self) -> Rect {
        let sprite = self.current_sprite().expect("Cell not found");
        Rect {
            x: (self.state_machine.context().position.x + sprite.sprite_source_size.x).into(),
            y: (self.state_machine.context().position.y + sprite.sprite_source_size.y).into(),
            width: sprite.frame.w.into(),
            height: sprite.frame.h.into(),
        }
    }

    fn draw(&self, renderer: &Renderer) {
        let sprite = self.current_sprite().expect("Sprite sheet cell not found");
        renderer.draw_image(
            &self.image,
            &Rect {
                x: sprite.frame.x.into(),
                y: sprite.frame.y.into(),
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            },
            &self.destination_box(),
        );
        renderer.draw_rect(&self.bounding_box());
    }

    fn frame_name(&self) -> String {
        format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            (self.state_machine.context().frame / 3) + 1
        )
    }

    fn jump(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Jump);
    }

    fn knock_out(&mut self) {
        self.state_machine = self.state_machine.transition(Event::KnockOut);
    }

    fn land_on(&mut self, position: i16) {
        self.state_machine = self.state_machine.transition(Event::Land(position));
    }

    fn position_y(&self) -> i16 {
        self.state_machine.context().position.y
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

    fn velocity_y(&self) -> i16 {
        self.state_machine.context().velocity.y
    }

    fn walking_speed(&self) -> i16 {
        self.state_machine.context().velocity.x
    }
}

// See p214. This could be implemented as a trait object instead
#[derive(Clone, Copy)]
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Falling(RedHatBoyState<Falling>),
    Jumping(RedHatBoyState<Jumping>),
    KnockedOut(RedHatBoyState<KnockedOut>),
    Running(RedHatBoyState<Running>),
    Sliding(RedHatBoyState<Sliding>),
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Jump,
    KnockOut,
    Land(i16),
    Run,
    Slide,
    Update,
}

impl RedHatBoyStateMachine {
    fn context(&self) -> &RedHatBoyContext {
        match self {
            RedHatBoyStateMachine::Falling(state) => &state.context(),
            RedHatBoyStateMachine::Idle(state) => &state.context(),
            RedHatBoyStateMachine::Jumping(state) => &state.context(),
            RedHatBoyStateMachine::KnockedOut(state) => &state.context(),
            RedHatBoyStateMachine::Running(state) => &state.context(),
            RedHatBoyStateMachine::Sliding(state) => &state.context(),
        }
    }

    fn frame_name(&self) -> &str {
        match self {
            RedHatBoyStateMachine::Falling(state) => state.frame_name(),
            RedHatBoyStateMachine::Idle(state) => state.frame_name(),
            RedHatBoyStateMachine::Jumping(state) => state.frame_name(),
            RedHatBoyStateMachine::KnockedOut(state) => state.frame_name(),
            RedHatBoyStateMachine::Running(state) => state.frame_name(),
            RedHatBoyStateMachine::Sliding(state) => state.frame_name(),
        }
    }

    fn transition(self, event: Event) -> Self {
        if event != Event::Update {
            log!("Event {event:?}");
        }

        match (self, event) {
            (RedHatBoyStateMachine::Falling(state), Event::Update) => state.update().into(),

            (RedHatBoyStateMachine::Idle(state), Event::Run) => state.run().into(),
            (RedHatBoyStateMachine::Idle(state), Event::Update) => state.update().into(),

            (RedHatBoyStateMachine::Running(state), Event::Jump) => state.jump().into(),
            (RedHatBoyStateMachine::Running(state), Event::KnockOut) => state.knock_out().into(),
            (RedHatBoyStateMachine::Running(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (RedHatBoyStateMachine::Running(state), Event::Slide) => state.slide().into(),
            (RedHatBoyStateMachine::Running(state), Event::Update) => state.update().into(),

            (RedHatBoyStateMachine::Jumping(state), Event::KnockOut) => state.knock_out().into(),
            (RedHatBoyStateMachine::Jumping(state), Event::Land(position)) => {
                state.land_on(position).into()
            }
            (RedHatBoyStateMachine::Jumping(state), Event::Update) => state.update().into(),

            (RedHatBoyStateMachine::Sliding(state), Event::KnockOut) => state.knock_out().into(),
            (RedHatBoyStateMachine::Sliding(state), Event::Land(position)) => {
                state.land_on(position).into()
            }

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

impl From<RedHatBoyState<Falling>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Falling>) -> Self {
        RedHatBoyStateMachine::Falling(state)
    }
}

impl From<RedHatBoyState<KnockedOut>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<KnockedOut>) -> Self {
        RedHatBoyStateMachine::KnockedOut(state)
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

struct Platform {
    image: HtmlImageElement,
    position: Point,
    sheet: Sheet,
}

impl Platform {
    fn new(sheet: Sheet, image: HtmlImageElement, position: Point) -> Self {
        Platform {
            image,
            position,
            sheet,
        }
    }

    fn bounding_boxes(&self) -> Vec<Rect> {
        const X_OFFSET: i16 = 60;
        const END_HEIGHT: i16 = 54;
        let destination_box = self.destination_box();
        let bounding_box_one = Rect {
            x: destination_box.x,
            y: destination_box.y,
            width: X_OFFSET,
            height: END_HEIGHT,
        };
        let bounding_box_two = Rect {
            x: destination_box.x + X_OFFSET,
            y: destination_box.y,
            width: destination_box.width - (X_OFFSET * 2),
            height: destination_box.height,
        };
        let bounding_box_three = Rect {
            x: destination_box.x + destination_box.width - X_OFFSET,
            y: destination_box.y,
            width: X_OFFSET,
            height: END_HEIGHT,
        };

        vec![bounding_box_one, bounding_box_two, bounding_box_three]
    }

    fn destination_box(&self) -> Rect {
        let platform = self
            .sheet
            .frames
            .get("13.png")
            .expect("13.png does not exist");

        Rect {
            x: self.position.x.into(),
            y: self.position.y.into(),
            width: (platform.frame.w * 3).into(),
            height: platform.frame.h.into(),
        }
    }

    fn draw(&self, renderer: &Renderer) {
        let platform = self
            .sheet
            .frames
            .get("13.png")
            .expect("13.png does not exist");
        renderer.draw_image(
            &self.image,
            &Rect {
                x: platform.frame.x.into(),
                y: platform.frame.y.into(),
                width: (platform.frame.w * 3).into(),
                height: platform.frame.h.into(),
            },
            &self.destination_box(),
        );

        self.bounding_boxes().into_iter().for_each(|b| {
            renderer.draw_rect(&b);
        });
    }
}
