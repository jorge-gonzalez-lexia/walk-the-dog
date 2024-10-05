mod context;
mod state_machine;
mod states;

use super::event_queue::{self, EventSubscriber, GameEvent};
use crate::engine::{
    rect::Rect,
    renderer::{DrawImageOptions, Renderer},
    sheet::{Cell, Sheet},
};
use state_machine::{DogStateMachine, Event};
use states::DogState;
use web_sys::HtmlImageElement;

pub struct Dog {
    image: HtmlImageElement,
    sprite_sheet: Sheet,
    state_machine: DogStateMachine,
}

impl Dog {
    pub fn new(
        sprite_sheet: Sheet,
        image: HtmlImageElement,
        event_publisher: event_queue::EventPublisher,
    ) -> Self {
        Dog {
            image,
            sprite_sheet,
            state_machine: DogStateMachine::Running(DogState::new(event_publisher)),
        }
    }

    pub fn info(&self) -> String {
        let ctx = self.state_machine.context();
        let bb = self.bounding_box();
        format!(
            "({},{},{},{}) v={:?} state={} ctx={}",
            bb.left(),
            bb.top(),
            bb.right(),
            bb.bottom(),
            ctx.velocity,
            self.state_machine.state_name(),
            ctx.info()
        )
    }

    pub fn moving_left(&self) -> bool {
        self.state_machine.context().moving_left()
    }

    pub fn moving_right(&self) -> bool {
        self.state_machine.context().moving_right()
    }

    pub fn moving_up(&self) -> bool {
        self.state_machine.context().velocity.y <= 0
    }

    pub fn reset(dog: Self) -> Self {
        Dog::new(
            dog.sprite_sheet,
            dog.image,
            dog.state_machine.context().event_publisher.clone(),
        )
    }

    pub fn bounding_box(&self) -> Rect {
        const Y_OFFSET: i16 = 8;
        const WIDTH_OFFSET: i16 = 50;
        let x_offset: i16 = if self.moving_left() { 20 } else { 35 };

        let mut bounding_box = self.destination_box();
        bounding_box.set_x(bounding_box.x() + x_offset);
        bounding_box.width -= WIDTH_OFFSET;
        bounding_box.position.y += Y_OFFSET;
        bounding_box.height -= Y_OFFSET;

        bounding_box
    }

    pub fn draw(&self, renderer: &Renderer) {
        let sprite = self.current_sprite();

        renderer.draw_image_ext(
            &self.image,
            &Rect::new_from_x_y(
                sprite.frame.x,
                sprite.frame.y,
                sprite.frame.w,
                sprite.frame.h,
            ),
            &self.destination_box(),
            DrawImageOptions {
                flip_horizontally: self.moving_left(),
            },
        );

        renderer.draw_rect(&self.bounding_box());
    }

    pub fn update(&mut self) {
        self.state_machine = self.state_machine.clone().update();
        // log!("Dog update {}", self.info());
    }

    fn current_sprite(&self) -> &Cell {
        let frame_name = self.state_machine.frame_name();
        self.sprite_sheet
            .frames
            .get(&frame_name)
            .unwrap_or_else(|| panic!("Frame '{frame_name}' not found"))
    }

    fn destination_box(&self) -> Rect {
        let sprite = self.current_sprite();

        Rect::new_from_x_y(
            self.state_machine.context().position.x + sprite.sprite_source_size.x,
            self.state_machine.context().position.y + sprite.sprite_source_size.y,
            sprite.frame.w,
            sprite.frame.h,
        )
    }

    fn transition(&mut self, event: Event, game_event: &GameEvent) {
        log!("Dog: GameEvent '{game_event:?}' => dog command '{event:?}'");

        self.state_machine = self.state_machine.clone().transition(event);
    }
}

impl EventSubscriber for Dog {
    fn name(&self) -> &str {
        "Dog"
    }

    fn process_event(&mut self, event: &GameEvent) {
        match event {
            GameEvent::BoyHitsObstacle => self.transition(Event::Worry, event),
            GameEvent::DogExitsPlatform { .. } => self.transition(Event::OffPlatform, event),
            GameEvent::DogHitMark { .. } => self.transition(Event::Jump, event),
            GameEvent::DogLandedOnGround => self.transition(Event::LandOnGround, event),
            GameEvent::DogLandedOnPlatform { platform_top, .. } => {
                self.transition(Event::LandOn(*platform_top), event)
            }
            GameEvent::DogTooClose | GameEvent::DogTooFar => {
                self.transition(Event::TurnAround, event)
            }
            GameEvent::GameStarted => self.transition(Event::Flee, event),
            _ => (),
        };
    }
}
