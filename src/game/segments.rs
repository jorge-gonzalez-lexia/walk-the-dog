use super::{event_queue::EventPublisher, obstacle::Obstacle, platform::Platform};
use crate::{
    engine::{
        image::Image,
        rect::{Point, Rect},
        sprite_sheet::SpriteSheet,
    },
    game::barrier::Barrier,
};
use rand::{thread_rng, Rng};
use std::rc::Rc;
use web_sys::HtmlImageElement;

const FIRST_PLATFORM: i16 = 240;
const HIGH_PLATFORM: i16 = 375;
const LOW_PLATFORM: i16 = 420;

const STONE_ON_GROUND: i16 = 546;

// -1 means random segments. Set to 0, 3, 4, 5 for testing specific segments
const REPEAT: i32 = -1;

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

    fn create_stone(&self, offset_x: i16) -> Barrier {
        Barrier::new(
            format!("b{}", self.id),
            Image::new(
                self.stone_image.clone(),
                Point {
                    x: offset_x,
                    y: STONE_ON_GROUND,
                },
            ),
            self.event_publisher.clone(),
        )
    }

    fn platform_and_stone(&self, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        let platform = self
            .create_floating_platform(Point {
                x: offset_x + 200,
                y: HIGH_PLATFORM,
            })
            .with_left_mark()
            .with_right_mark();

        let stone = self.create_stone(offset_x + 350);

        vec![Box::new(platform), Box::new(stone)]
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
        vec![Box::new(
            self.create_stone(offset_x + 150)
                .with_left_mark()
                .with_right_mark(),
        )]
    }

    fn stone_and_platform(&self, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        let stone = self.create_stone(offset_x + 130).with_left_mark();

        let platform = self
            .create_floating_platform(Point {
                x: offset_x + FIRST_PLATFORM,
                y: LOW_PLATFORM,
            })
            .with_right_mark();

        vec![Box::new(stone), Box::new(platform)]
    }

    fn stone_on_platform(&self, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        let stone = self.create_stone(offset_x + 390);
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
