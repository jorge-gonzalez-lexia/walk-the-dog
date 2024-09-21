use super::{dog::Dog, obstacle::Obstacle, red_hat_boy::RedHatBoy};
use crate::engine::image::Image;

pub struct Barrier {
    image: Image,
}

impl Barrier {
    pub fn new(image: Image) -> Self {
        Barrier { image }
    }
}

impl Obstacle for Barrier {
    fn check_intersection(&self, boy: &mut RedHatBoy) {
        if boy.bounding_box().intersects(self.image.bounding_box()) {
            boy.knock_out()
        }
    }

    fn draw(&self, renderer: &crate::engine::renderer::Renderer) {
        self.image.draw(renderer);
    }

    fn move_horizontally(&mut self, x: i16) {
        self.image.move_horizontally(x);
    }

    fn navigate(&self, dog: &mut Dog) {
        let mark = self.image.bounding_box().left() - 20;
        let r_mark = self.right();
        if dog.bounding_box().right() >= mark && dog.bounding_box().left() <= r_mark {
            // log!("nav barrier {}", dog.info());
            dog.navigate(dog.bounding_box().bottom());
        }
    }

    fn right(&self) -> i16 {
        self.image.right()
    }
}
