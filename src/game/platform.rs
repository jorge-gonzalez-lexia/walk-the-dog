use super::obstacle::Obstacle;
use crate::engine::{
    rect::{Point, Rect},
    renderer::Renderer,
    sheet::Sheet,
};
use web_sys::HtmlImageElement;

pub struct Platform {
    image: HtmlImageElement,
    pub position: Point,
    sheet: Sheet,
}

impl Platform {
    pub fn new(sheet: Sheet, image: HtmlImageElement, position: Point) -> Self {
        Platform {
            image,
            position,
            sheet,
        }
    }

    pub fn bounding_boxes(&self) -> Vec<Rect> {
        const X_OFFSET: i16 = 60;
        const END_HEIGHT: i16 = 54;
        let destination_box = self.destination_box();
        let bounding_box_one = Rect::new(destination_box.position, X_OFFSET, END_HEIGHT);
        let bounding_box_two = Rect::new_from_x_y(
            destination_box.x() + X_OFFSET,
            destination_box.y(),
            destination_box.width - (X_OFFSET * 2),
            destination_box.height,
        );
        let bounding_box_three = Rect::new_from_x_y(
            destination_box.right() - X_OFFSET,
            destination_box.y(),
            X_OFFSET,
            END_HEIGHT,
        );

        vec![bounding_box_one, bounding_box_two, bounding_box_three]
    }

    pub fn destination_box(&self) -> Rect {
        let platform = self
            .sheet
            .frames
            .get("13.png")
            .expect("13.png does not exist");

        Rect::new(self.position, platform.frame.w * 3, platform.frame.h)
    }
}

impl Obstacle for Platform {
    fn check_intersection(&self, boy: &mut super::red_hat_boy::RedHatBoy) {
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
        let platform = self
            .sheet
            .frames
            .get("13.png")
            .expect("13.png does not exist");
        renderer.draw_image(
            &self.image,
            &&Rect::new_from_x_y(
                platform.frame.x,
                platform.frame.y,
                platform.frame.w * 3,
                platform.frame.h,
            ),
            &self.destination_box(),
        );

        self.bounding_boxes().into_iter().for_each(|b| {
            renderer.draw_rect(&b);
        });
    }

    fn move_horizontally(&mut self, x: i16) {
        self.position.x += x;
    }
}
