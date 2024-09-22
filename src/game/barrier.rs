use super::{dog::Dog, obstacle::Obstacle, red_hat_boy::RedHatBoy};
use crate::engine::{
    image::Image,
    rect::{Point, Rect},
    renderer::Renderer,
};

pub struct Barrier {
    image: Image,
    has_mark_left: bool,
    has_mark_right: bool,
}

impl Barrier {
    pub fn new(image: Image) -> Self {
        Barrier {
            image,
            has_mark_left: false,
            has_mark_right: false,
        }
    }

    pub fn with_left_mark(mut self) -> Self {
        self.has_mark_left = true;

        self
    }

    pub fn with_right_mark(mut self) -> Self {
        self.has_mark_right = true;

        self
    }

    fn draw_marks(&self, renderer: &Renderer) {
        if self.has_mark_left {
            renderer.draw_rect_colored(&self.mark_left(), "#000000");
        }

        if self.has_mark_right {
            renderer.draw_rect_colored(&self.mark_right(), "#FFFF00");
        }
    }

    fn mark_left(&self) -> Rect {
        Rect::new(
            Point {
                x: self.image.bounding_box().x() - 20,
                y: self.image.bounding_box().y(),
            },
            1,
            self.image.bounding_box().height,
        )
    }

    fn mark_right(&self) -> Rect {
        Rect::new(
            Point {
                x: self.image.bounding_box().right() + 20,
                y: self.image.bounding_box().y(),
            },
            1,
            50,
        )
    }

    fn on_left_mark(&self, dog: &Dog) -> bool {
        if !self.has_mark_left || !dog.moving_right() {
            return false;
        }

        dog.bounding_box().intersects(&self.mark_left())
    }

    fn on_right_mark(&self, dog: &Dog) -> bool {
        if !self.has_mark_right || !dog.moving_left() {
            return false;
        }

        dog.bounding_box().intersects(&self.mark_right())
    }
}

impl Obstacle for Barrier {
    fn check_intersection(&self, boy: &mut RedHatBoy) {
        if boy.bounding_box().intersects(self.image.bounding_box()) {
            boy.knock_out()
        }
    }

    fn draw(&self, renderer: &Renderer) {
        self.image.draw(renderer);
        self.draw_marks(renderer);
    }

    fn move_horizontally(&mut self, x: i16) {
        self.image.move_horizontally(x);
    }

    fn navigate(&self, dog: &mut Dog) {
        // log!(
        //     "Stone mark left={} x={} right={} {}",
        //     self.on_left_mark(dog),
        //     self.mark_left().left(),
        //     self.on_right_mark(dog),
        //     dog.info()
        // );
        if self.on_left_mark(dog) || self.on_right_mark(dog) {
            dog.jump();
        }
    }

    fn right(&self) -> i16 {
        self.image.right()
    }
}
