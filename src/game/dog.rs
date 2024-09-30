mod context;
mod state_machine;
mod states;

use super::event_queue::{self, GameEvent};
use crate::engine::{
    rect::Rect,
    renderer::{DrawImageOptions, Renderer},
    sheet::{Cell, Sheet},
};
use context::DOG_GROUND;
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

    pub fn flee(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Flee);
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

    pub fn jump(&mut self) {
        if self.state_machine.context().velocity.y < 0 {
            return;
        }

        self.state_machine = self.state_machine.clone().transition(Event::Jump);
    }

    pub fn moving_left(&self) -> bool {
        self.state_machine.context().velocity.x < 0
    }

    pub fn moving_right(&self) -> bool {
        self.state_machine.context().velocity.x >= 0
    }

    pub fn off_platform(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::OffPlatform);
    }

    pub fn on_platform(&mut self, top: i16) {
        assert!(self.state_machine.context().velocity.y > 0);

        if self.state_machine.context().floor == DOG_GROUND {
            // log!("Hit platform {} floor={DOG_FLOOR}", self.info());
            self.state_machine = self.state_machine.clone().transition(Event::Land(top))
        }
    }

    pub fn process_event(&mut self, event: &GameEvent) {
        log!("Dog: process game event {event:?}");
        self.state_machine = match event {
            GameEvent::DogLanded => self.state_machine.clone().transition(Event::LandOnGround),
            GameEvent::DogTooClose => self.state_machine.clone().transition(Event::TurnAround),
            GameEvent::DogTooFar => self.state_machine.clone().transition(Event::TurnAround),
        }
    }

    pub fn reset(dog: Self) -> Self {
        Dog::new(
            dog.sprite_sheet,
            dog.image,
            dog.state_machine.context().event_publisher.clone(),
        )
    }

    pub fn worry(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Worry);
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
}
