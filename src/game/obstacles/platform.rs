use super::{
    obstacle_mark::{ObstacleMark, ObstacleMarkDirection},
    Obstacle, ObstacleMarkFactory,
};
use crate::{
    engine::{
        rect::{Point, Rect},
        renderer::Renderer,
        sheet::Cell,
        sprite_sheet::SpriteSheet,
    },
    game::{
        dog::Dog,
        event_queue::{EventPublisher, GameEvent},
        red_hat_boy::RedHatBoy,
    },
};
use std::rc::Rc;

const MARK_OFFSET: i16 = 80;

#[derive(Debug)]
pub struct Platform {
    pub position: Point,
    bounding_boxes: Vec<Rect>,
    event_publisher: EventPublisher,
    /// True when dog is running on platform
    has_dog: bool,
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
            id,
            has_dog: false,
            position,
            sheet,
            sprites,
        }
    }

    pub fn bounding_boxes(&self) -> &Vec<Rect> {
        &self.bounding_boxes
    }

    fn on_platform(&self, dog: &Dog) -> bool {
        self.bounding_boxes()
            .iter()
            .any(|b| dog.bounding_box().intersects(b))
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

    fn process_event(&mut self, event: &GameEvent) {
        match event {
            GameEvent::DogExitsPlatform if self.has_dog => {
                log!("Platform {}: Dog exited platform", self.id);
                self.has_dog = false;
            }
            GameEvent::DogLandedOnPlatform { id, .. } if *id == self.id && !self.has_dog => {
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

impl ObstacleMarkFactory for Platform {
    fn mark_left(&self) -> ObstacleMark {
        ObstacleMark::new(
            Point {
                x: self.position.x - MARK_OFFSET,
                y: self.position.y,
            },
            ObstacleMarkDirection::Left,
            self.id.clone(),
            self.event_publisher.clone(),
        )
    }

    fn mark_right(&self) -> ObstacleMark {
        ObstacleMark::new(
            Point {
                x: self.right() + MARK_OFFSET,
                y: self.position.y,
            },
            ObstacleMarkDirection::Right,
            self.id.clone(),
            self.event_publisher.clone(),
        )
    }
}
