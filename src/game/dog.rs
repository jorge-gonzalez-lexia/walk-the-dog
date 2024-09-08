mod context;
mod state_machine;
mod states;

use crate::engine::{
    rect::Rect,
    renderer::Renderer,
    sheet::{Cell, Sheet},
};
use state_machine::DogStateMachine;
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

    pub fn reset(dog: Self) -> Self {
        Dog::new(dog.sprite_sheet, dog.image)
    }

    // TODO likely not needed and can just use destination box since dog will never hit an obstacle
    pub fn bounding_box(&self) -> Rect {
        self.destination_box()
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
        format!("rr_000.png")
    }
}
