pub mod context;
mod state_machine;
mod states;

use crate::engine::{
    audio::Audio,
    rect::Rect,
    renderer::Renderer,
    sheet::{Cell, Sheet},
};
use context::Sfx;
use state_machine::{Event, RedHatBoyStateMachine};
use states::RedHatBoyState;
use web_sys::HtmlImageElement;

pub struct RedHatBoy {
    image: HtmlImageElement,
    sprite_sheet: Sheet,
    state_machine: RedHatBoyStateMachine,
}

impl RedHatBoy {
    pub fn new(audio: Audio, sfx: Sfx, sprite_sheet: Sheet, image: HtmlImageElement) -> Self {
        RedHatBoy {
            image,
            sprite_sheet,
            state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::new(audio, sfx)),
        }
    }

    pub fn reset(boy: Self) -> Self {
        RedHatBoy::new(
            boy.state_machine.context().audio.clone(),
            boy.state_machine.context().sfx.clone(),
            boy.sprite_sheet,
            boy.image,
        )
    }

    pub fn bounding_box(&self) -> Rect {
        const X_OFFSET: i16 = 18;
        const Y_OFFSET: i16 = 14;
        const WIDTH_OFFSET: i16 = 28;

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

    pub fn is_running(&self) -> bool {
        matches!(
            self.state_machine,
            RedHatBoyStateMachine::Jumping(_) | RedHatBoyStateMachine::Running(_)
        )
    }

    pub fn jump(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Jump);
    }

    pub fn knock_out(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::KnockOut);
    }

    pub fn knocked_out(&self) -> bool {
        self.state_machine.knocked_out()
    }

    pub fn land_on(&mut self, position: i16) {
        self.state_machine = self.state_machine.clone().transition(Event::Land(position));
    }

    pub fn position_y(&self) -> i16 {
        self.state_machine.context().position.y
    }

    pub fn run_right(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Run);
    }

    pub fn slide(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Slide);
    }

    pub fn update(&mut self) {
        self.state_machine = self.state_machine.clone().update();
    }

    pub fn velocity_y(&self) -> i16 {
        self.state_machine.context().velocity.y
    }

    pub fn walking_speed(&self) -> i16 {
        self.state_machine.context().velocity.x
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
        format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            (self.state_machine.context().frame / 3) + 1
        )
    }
}
