use super::{
    event_queue::EventPublisher,
    obstacles::{barrier::Barrier, platform::Platform, Obstacle, ObstacleMarkFactory},
};
use crate::engine::{
    image::Image,
    rect::{Point, Rect},
    sprite_sheet::SpriteSheet,
};
use rand::{thread_rng, Rng};
use std::rc::Rc;
use web_sys::HtmlImageElement;

const FIRST_PLATFORM: i16 = 240;
const HIGH_PLATFORM: i16 = 375;
const LOW_PLATFORM: i16 = 420;

const STONE_ON_GROUND: i16 = 546;
const STONE_ON_PLATFORM: i16 = 322;

const STONE_AND_PLATFORM_SEGMENT_ID: i32 = 4;

pub struct SegmentFactory {
    event_publisher: EventPublisher,
    id: i32,
    obstacle_sheet: Rc<SpriteSheet>,
    stone_image: HtmlImageElement,
}

impl SegmentFactory {
    pub fn new(
        sprite_sheet: Rc<SpriteSheet>,
        stone_image: HtmlImageElement,
        event_publisher: EventPublisher,
    ) -> Self {
        SegmentFactory {
            event_publisher,
            id: 0,
            obstacle_sheet: sprite_sheet.clone(),
            stone_image,
        }
    }

    pub fn first(&mut self) -> Vec<Box<dyn Obstacle>> {
        const OFFSET_X: i16 = 0;

        if REPEAT >= 0 {
            self.select(REPEAT, OFFSET_X)
        } else {
            self.select(STONE_AND_PLATFORM_SEGMENT_ID, OFFSET_X)
        }
    }

    pub fn random(&mut self, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        let mut rng = thread_rng();
        let next_segment = if REPEAT >= 0 {
            REPEAT
        } else {
            rng.gen_range(0..6)
        };

        self.select(next_segment, offset_x)
    }

    fn create_floating_platform(&self, position: Point) -> Platform {
        Platform::new(
            format!("p{}", self.id.to_string()),
            self.obstacle_sheet.clone(),
            position,
            &FLOATING_PLATFORM_SPRITES,
            &FLOATING_PLATFORM_BOUNDING_BOXES,
            self.event_publisher.clone(),
        )
    }

    fn create_stone(&self, x: i16, y: i16) -> Barrier {
        Barrier::new(
            format!("b{}", self.id),
            Image::new(self.stone_image.clone(), Point { x, y }),
            self.event_publisher.clone(),
        )
    }

    fn platform_and_stone(&self, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        let platform = self.create_floating_platform(Point {
            x: offset_x + 200,
            y: HIGH_PLATFORM,
        });
        let stone = self.create_stone(offset_x + 350, STONE_ON_GROUND);

        let mark_left = platform.mark_left();
        let mark_right = platform.mark_right();

        vec![
            Box::new(mark_left),
            Box::new(platform),
            Box::new(stone),
            Box::new(mark_right),
        ]
    }

    fn platform_high(&self, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        let platform = self.create_floating_platform(Point {
            x: offset_x + 200,
            y: HIGH_PLATFORM,
        });

        vec![Box::new(platform)]
    }

    fn platform_low(&self, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        let platform = self.create_floating_platform(Point {
            x: offset_x + 200,
            y: LOW_PLATFORM,
        });

        vec![Box::new(platform)]
    }

    fn stone(&self, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        let stone = self.create_stone(offset_x + 150, STONE_ON_GROUND);

        let mark_left = stone.mark_left();
        let mark_right = stone.mark_right();

        vec![Box::new(mark_left), Box::new(stone), Box::new(mark_right)]
    }

    fn stone_and_platform(&self, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        let stone = self.create_stone(offset_x + 130, STONE_ON_GROUND);
        let platform = self.create_floating_platform(Point {
            x: offset_x + FIRST_PLATFORM,
            y: LOW_PLATFORM,
        });

        let mark_left = stone.mark_left();
        let mark_right = platform.mark_right();

        vec![
            Box::new(mark_left),
            Box::new(stone),
            Box::new(platform),
            Box::new(mark_right),
        ]
    }

    fn stone_on_platform(&self, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        let stone = self.create_stone(offset_x + 390, STONE_ON_PLATFORM);
        let platform = self.create_floating_platform(Point {
            x: offset_x + 200,
            y: HIGH_PLATFORM,
        });

        vec![Box::new(stone), Box::new(platform)]
    }

    fn select(&mut self, segment: i32, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        self.id += 1;

        match segment {
            0 => self.platform_and_stone(offset_x),
            1 => self.platform_high(offset_x),
            2 => self.platform_low(offset_x),
            3 => self.stone(offset_x),
            4 => self.stone_and_platform(offset_x),
            5 => self.stone_on_platform(offset_x),

            _ => vec![],
        }
    }
}

const FLOATING_PLATFORM_BOUNDING_BOXES: [Rect; 3] = [
    Rect::new_from_x_y(0, 0, 60, 54),
    Rect::new_from_x_y(60, 0, 384 - (60 * 2), 93),
    Rect::new_from_x_y(384 - 60, 0, 60, 54),
];
const FLOATING_PLATFORM_SPRITES: [&str; 3] = ["13.png", "14.png", "15.png"];

// -1 means random segments. Set to 0, 3, 4, 5 for testing specific segments
const REPEAT: i32 = -1;
