use super::{dog::Dog, obstacle::Obstacle, red_hat_boy::RedHatBoy};
use crate::engine::{
    rect::{Point, Rect},
    renderer::Renderer,
    sheet::Cell,
    sprite_sheet::SpriteSheet,
};
use std::rc::Rc;

pub struct Platform {
    pub position: Point,
    bounding_boxes: Vec<Rect>,
    sheet: Rc<SpriteSheet>,
    sprites: Vec<Cell>,
}

impl Platform {
    pub fn new(
        sheet: Rc<SpriteSheet>,
        position: Point,
        sprite_names: &[&str],
        bounding_boxes: &[Rect],
    ) -> Self {
        let sprites = sprite_names
            .iter()
            .filter_map(|sprite_name| sheet.cell(sprite_name).cloned())
            .collect();
        let bounding_boxes = bounding_boxes
            .iter()
            .map(|b| Rect::new_from_x_y(b.x() + position.x, b.y() + position.y, b.width, b.height))
            .collect();

        Platform {
            bounding_boxes,
            position,
            sheet,
            sprites,
        }
    }

    pub fn bounding_boxes(&self) -> &Vec<Rect> {
        &self.bounding_boxes
    }
}

impl Obstacle for Platform {
    fn check_intersection(&self, boy: &mut RedHatBoy) {
        if let Some(box_to_land_on) = self
            .bounding_boxes()
            .iter()
            .find(|&b| boy.bounding_box().intersects(b))
        {
            if boy.velocity_y() > 0 && boy.position_y() < self.position.y {
                boy.land_on(box_to_land_on.top());
            } else {
                boy.knock_out();
            }
        }
    }

    fn draw(&self, renderer: &Renderer) {
        let mut x = 0;

        self.sprites.iter().for_each(|sprite| {
            self.sheet.draw(
                renderer,
                &Rect::new_from_x_y(
                    sprite.frame.x,
                    sprite.frame.y,
                    sprite.frame.w,
                    sprite.frame.h,
                ),
                &Rect::new_from_x_y(
                    self.position.x + x,
                    self.position.y,
                    sprite.frame.w,
                    sprite.frame.h,
                ),
            );

            x += sprite.frame.w;
        });

        self.bounding_boxes
            .iter()
            .for_each(|b| renderer.draw_rect(b));
    }

    fn move_horizontally(&mut self, x: i16) {
        self.position.x += x;
        self.bounding_boxes
            .iter_mut()
            .for_each(|b| b.set_x(b.position.x + x));
    }

    fn navigate(&self, dog: &mut Dog) {
        let mark = self.position.x - 20; // TODO 20px prior to left bb
        let r_mark = self.right(); // TODO 20px after width
        if dog.bounding_box().right() >= mark && dog.bounding_box().left() <= r_mark {
            log!("{} platform y={}", dog.info(), self.position.y);
            dog.navigate(self.position.y);
        }
    }

    fn right(&self) -> i16 {
        self.bounding_boxes()
            .last()
            .unwrap_or(&Rect::default())
            .right()
    }
}
