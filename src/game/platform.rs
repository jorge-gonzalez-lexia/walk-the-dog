use super::{
    dog::Dog,
    event_queue::{EventPublisher, GameEvent},
    obstacle::Obstacle,
    red_hat_boy::RedHatBoy,
    HEIGHT,
};
use crate::engine::{
    rect::{Point, Rect},
    renderer::Renderer,
    sheet::Cell,
    sprite_sheet::SpriteSheet,
};
use std::rc::Rc;

const MARK_OFFSET: i16 = 80;

#[derive(Debug)]
pub struct Platform {
    pub position: Point,
    bounding_boxes: Vec<Rect>,
    dog_on_mark: bool,
    event_publisher: EventPublisher,
    /// True when dog is running on platform
    has_dog: bool,
    has_mark_left: bool,
    has_mark_right: bool,
    id: String,
    sheet: Rc<SpriteSheet>,
    sprites: Vec<Cell>,
}

impl Platform {
    pub fn new(
        id: String,
        sheet: Rc<SpriteSheet>,
        position: Point,
        sprite_names: &[&str],
        bounding_boxes: &[Rect],
        event_publisher: EventPublisher,
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
            event_publisher,
            dog_on_mark: false,
            id,
            has_dog: false,
            has_mark_left: false,
            has_mark_right: false,
            position,
            sheet,
            sprites,
        }
    }

    pub fn bounding_boxes(&self) -> &Vec<Rect> {
        &self.bounding_boxes
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

    fn on_platform(&self, dog: &Dog) -> bool {
        self.bounding_boxes()
            .iter()
            .any(|b| dog.bounding_box().intersects(b))
    }

    fn mark_left(&self) -> Rect {
        Rect::new(
            Point {
                x: self.position.x - MARK_OFFSET,
                y: self.position.y,
            },
            1,
            HEIGHT - self.position.y,
        )
    }

    fn mark_right(&self) -> Rect {
        Rect::new(
            Point {
                x: self.right() + MARK_OFFSET,
                y: self.position.y,
            },
            1,
            HEIGHT - self.position.y,
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

    #[allow(dead_code)]
    fn hit_info(&self, dog: &Dog) -> String {
        let bb = self.bounding_boxes().last().unwrap();

        format!(
            "has_dog={} platform left,top=({},{}) right,bottom({}.{})\nDog {}",
            self.has_dog,
            self.position.x,
            self.position.y,
            bb.right(),
            bb.bottom(),
            dog.info()
        )
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
                self.event_publisher.publish(GameEvent::BoyHitsObstacle);
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
        self.draw_marks(renderer);
    }

    fn move_horizontally(&mut self, x: i16) {
        self.position.x += x;
        self.bounding_boxes
            .iter_mut()
            .for_each(|b| b.set_x(b.position.x + x));
    }

    /// Publish GameEvents when Dog interacts with the platform or a mark that
    /// should trigger the Dog to jump.
    fn navigate(&mut self, dog: &Dog) {
        let is_on_mark = self.on_left_mark(dog) || self.on_right_mark(dog);
        if is_on_mark && !self.dog_on_mark {
            self.event_publisher.publish(GameEvent::DogHitMark {
                id: self.id.clone(),
            });
        }
        if !is_on_mark && self.dog_on_mark {
            self.event_publisher.publish(GameEvent::DogOffMark {
                id: self.id.clone(),
            });
        }

        let is_on_platform = self.on_platform(dog);
        if is_on_platform && !self.has_dog {
            self.event_publisher
                .publish(GameEvent::DogLandedOnPlatform {
                    id: self.id.clone(),
                    platform_top: self.position.y,
                });
        }

        if !is_on_platform && self.has_dog {
            self.event_publisher.publish(GameEvent::DogExitsPlatform);
        }
    }

    fn process_event(&mut self, event: &super::event_queue::GameEvent) {
        match event {
            GameEvent::DogExitsPlatform => {
                log!("Platform {}: Dog exited platform", self.id);
                self.has_dog = false;
            }
            GameEvent::DogHitMark { id } if *id == self.id => {
                log!("Platform {}: Dog hit mark", self.id);
                self.dog_on_mark = true;
            }
            GameEvent::DogOffMark { id } if *id == self.id => {
                log!("Platform {}: Dog off mark", self.id);
                self.dog_on_mark = false;
            }
            GameEvent::DogLandedOnPlatform { id, .. } if *id == self.id => {
                log!("Platform {}: Dog landed on platform", self.id);
                self.has_dog = true
            }
            _ => (),
        }
    }

    fn right(&self) -> i16 {
        self.bounding_boxes()
            .last()
            .unwrap_or(&Rect::default())
            .right()
    }
}
