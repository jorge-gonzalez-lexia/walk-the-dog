use super::{obstacle::Obstacle, platform::Platform};
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
const STONE_ON_PLATFORM: i16 = 322;

const REPEAT: i32 = -1;

const STONE_AND_PLATFORM_SEGMENT_ID: i32 = 4;

pub struct SegmentFactory {
    obstacle_sheet: Rc<SpriteSheet>,
    stone_image: HtmlImageElement,
}

impl SegmentFactory {
    pub fn new(sprite_sheet: Rc<SpriteSheet>, stone_image: HtmlImageElement) -> Self {
        SegmentFactory {
            obstacle_sheet: sprite_sheet.clone(),
            stone_image,
        }
    }

    pub fn first(&self) -> Vec<Box<dyn Obstacle>> {
        const OFFSET_X: i16 = 0;

        if REPEAT >= 0 {
            self.select(REPEAT, OFFSET_X)
        } else {
            self.select(STONE_AND_PLATFORM_SEGMENT_ID, OFFSET_X)
        }
    }

    pub fn random(&self, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        let mut rng = thread_rng();
        let next_segment = if REPEAT >= 0 {
            REPEAT
        } else {
            rng.gen_range(0..6)
        };

        self.select(next_segment, offset_x)
    }

    fn select(&self, segment: i32, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
        let obstacle_sheet = self.obstacle_sheet.clone();

        match segment {
            0 => platform_and_stone(self.stone_image.clone(), obstacle_sheet, offset_x),
            1 => platform_high(obstacle_sheet, offset_x),
            2 => platform_low(obstacle_sheet, offset_x),
            3 => stone(self.stone_image.clone(), offset_x),
            4 => stone_and_platform(self.stone_image.clone(), obstacle_sheet, offset_x),
            5 => stone_on_platform(self.stone_image.clone(), obstacle_sheet, offset_x),

            _ => vec![],
        }
    }
}

pub fn platform_and_stone(
    stone_image: HtmlImageElement,
    sprite_sheet: Rc<SpriteSheet>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    const INITIAL_STONE_OFFSET: i16 = 350;
    const PLATFORM_OFFSET: i16 = 200;

    let platform = create_floating_platform(
        sprite_sheet,
        Point {
            x: offset_x + PLATFORM_OFFSET,
            y: HIGH_PLATFORM,
        },
    )
    .with_left_mark()
    .with_right_mark();

    let stone = Barrier::new(Image::new(
        stone_image,
        Point {
            x: offset_x + INITIAL_STONE_OFFSET,
            y: STONE_ON_GROUND,
        },
    ));

    vec![Box::new(platform), Box::new(stone)]
}

pub fn platform_high(sprite_sheet: Rc<SpriteSheet>, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
    let platform = create_floating_platform(
        sprite_sheet,
        Point {
            x: offset_x + 200,
            y: HIGH_PLATFORM,
        },
    );

    vec![Box::new(platform)]
}

pub fn platform_low(sprite_sheet: Rc<SpriteSheet>, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
    let platform = create_floating_platform(
        sprite_sheet,
        Point {
            x: offset_x + 200,
            y: LOW_PLATFORM,
        },
    );

    vec![Box::new(platform)]
}

pub fn stone(stone_image: HtmlImageElement, offset_x: i16) -> Vec<Box<dyn Obstacle>> {
    const INITIAL_STONE_OFFSET: i16 = 150;
    let stone = Barrier::new(Image::new(
        stone_image,
        Point {
            x: offset_x + INITIAL_STONE_OFFSET,
            y: STONE_ON_GROUND,
        },
    ))
    .with_left_mark()
    .with_right_mark();

    vec![Box::new(stone)]
}

pub fn stone_and_platform(
    stone_image: HtmlImageElement,
    sprite_sheet: Rc<SpriteSheet>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    const INITIAL_STONE_OFFSET: i16 = 130;
    let stone = Barrier::new(Image::new(
        stone_image,
        Point {
            x: offset_x + INITIAL_STONE_OFFSET,
            y: STONE_ON_GROUND,
        },
    ))
    .with_left_mark();

    let platform = create_floating_platform(
        sprite_sheet,
        Point {
            x: offset_x + FIRST_PLATFORM,
            y: LOW_PLATFORM,
        },
    )
    .with_right_mark();

    vec![Box::new(stone), Box::new(platform)]
}

pub fn stone_on_platform(
    stone_image: HtmlImageElement,
    sprite_sheet: Rc<SpriteSheet>,
    offset_x: i16,
) -> Vec<Box<dyn Obstacle>> {
    const INITIAL_STONE_OFFSET: i16 = 390;
    let stone = Barrier::new(Image::new(
        stone_image,
        Point {
            x: offset_x + INITIAL_STONE_OFFSET,
            y: STONE_ON_PLATFORM,
        },
    ));
    let platform = create_floating_platform(
        sprite_sheet,
        Point {
            x: offset_x + 200,
            y: HIGH_PLATFORM,
        },
    );

    vec![Box::new(stone), Box::new(platform)]
}

const FLOATING_PLATFORM_BOUNDING_BOXES: [Rect; 3] = [
    Rect::new_from_x_y(0, 0, 60, 54),
    Rect::new_from_x_y(60, 0, 384 - (60 * 2), 93),
    Rect::new_from_x_y(384 - 60, 0, 60, 54),
];
const FLOATING_PLATFORM_SPRITES: [&str; 3] = ["13.png", "14.png", "15.png"];

fn create_floating_platform(sprite_sheet: Rc<SpriteSheet>, position: Point) -> Platform {
    Platform::new(
        sprite_sheet,
        position,
        &FLOATING_PLATFORM_SPRITES,
        &FLOATING_PLATFORM_BOUNDING_BOXES,
    )
}
