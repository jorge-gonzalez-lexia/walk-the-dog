mod context;
mod state_machine;
mod states;

use crate::engine::{
    rect::Rect,
    renderer::Renderer,
    sheet::{Cell, Sheet},
};
use context::{DOG_FLOOR, DOG_HEIGHT};
use state_machine::{DogStateMachine, Event};
use states::DogState;
use web_sys::HtmlImageElement;

pub struct Dog {
    image: HtmlImageElement,
    sprite_sheet: Sheet,
    state_machine: DogStateMachine,
}

impl Dog {
    pub fn new(sprite_sheet: Sheet, image: HtmlImageElement) -> Self {
        Dog {
            image,
            sprite_sheet,
            state_machine: DogStateMachine::Running(DogState::new()),
        }
    }

    pub fn flee(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Flee);
    }

    pub fn jump(&mut self) {
        if self.state_machine.context().velocity.y < 0 {
            return;
        }

        self.state_machine = self.state_machine.clone().transition(Event::Jump);
    }

    pub fn info(&self) -> String {
        let ctx = self.state_machine.context();
        let bb = self.bounding_box();
        format!(
            "({},{},{},{}) v={:?}",
            bb.left(),
            bb.top(),
            bb.right(),
            bb.bottom(),
            ctx.velocity,
        )
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
        if self.state_machine.context().floor == DOG_FLOOR {
            self.state_machine = self.state_machine.clone().transition(Event::Land(top))
        }
    }

    pub fn reset(dog: Self) -> Self {
        Dog::new(dog.sprite_sheet, dog.image)
    }

    pub fn worry(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Worry);
    }

    pub fn bounding_box(&self) -> Rect {
        const X_OFFSET: i16 = 35;
        const Y_OFFSET: i16 = 8;
        const WIDTH_OFFSET: i16 = 50;

        let mut bounding_box = self.destination_box();
        bounding_box.set_x(bounding_box.x() + X_OFFSET);
        bounding_box.width -= WIDTH_OFFSET;
        bounding_box.position.y += Y_OFFSET;
        bounding_box.height -= Y_OFFSET;

        bounding_box
    }

    pub fn draw(&self, renderer: &Renderer) {
        let sprite = self.current_sprite();
        renderer.draw_image(
            &self.image,
            &Rect::new_from_x_y(
                sprite.frame.x,
                sprite.frame.y,
                sprite.frame.w,
                sprite.frame.h,
            ),
            &self.destination_box(),
        );
        renderer.draw_rect(&self.bounding_box());
    }

    pub fn update(&mut self) {
        self.state_machine = self.state_machine.clone().update();
    }

    fn current_sprite(&self) -> &Cell {
        let frame_name = self.frame_name();
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

    fn frame_name(&self) -> String {
        let animation_frame = self.state_machine.context().frame / 3;
        format!("rr_{animation_frame:03}.png")
    }
}
