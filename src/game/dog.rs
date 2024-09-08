use crate::engine::sheet::Sheet;
use web_sys::HtmlImageElement;

pub struct Dog {
    image: HtmlImageElement,
    sprite_sheet: Sheet,
}

impl Dog {
    pub fn new(sprite_sheet: Sheet, image: HtmlImageElement) -> Self {
        Dog {
            image,
            sprite_sheet,
        }
    }

    pub fn reset(dog: Self) -> Self {
        Dog::new(dog.sprite_sheet, dog.image)
    }
}
