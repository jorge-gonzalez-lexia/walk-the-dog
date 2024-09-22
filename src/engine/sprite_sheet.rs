use super::{
    rect::Rect,
    renderer::Renderer,
    sheet::{Cell, Sheet},
};
use web_sys::HtmlImageElement;

#[derive(Debug)]
pub struct SpriteSheet {
    image: HtmlImageElement,
    sheet: Sheet,
}

impl SpriteSheet {
    pub fn new(sheet: Sheet, image: HtmlImageElement) -> Self {
        SpriteSheet { image, sheet }
    }

    pub fn cell(&self, name: &str) -> Option<&Cell> {
        self.sheet.frames.get(name)
    }

    pub fn draw(&self, renderer: &Renderer, source: &Rect, destination: &Rect) {
        renderer.draw_image(&self.image, source, destination);
    }
}
