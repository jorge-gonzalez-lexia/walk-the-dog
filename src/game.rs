use crate::{
    browser,
    engine::{self, Game, Rect, Renderer},
};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use web_sys::HtmlImageElement;

#[derive(Deserialize)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}

pub struct WalkTheDog {
    image: Option<HtmlImageElement>,
    sheet: Option<Sheet>,
    frame: u8,
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    fn draw(&self, renderer: &Renderer) {
        let frame_name = format!("Run ({}).png", self.frame + 1);
        let sprite = self
            .sheet
            .as_ref()
            .and_then(|sheet| sheet.frames.get(&frame_name))
            .expect("Cell not found");
        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });
        self.image.as_ref().map(|image| {
            renderer.draw_image(
                &image,
                &Rect {
                    x: sprite.frame.x.into(),
                    y: sprite.frame.y.into(),
                    width: sprite.frame.w.into(),
                    height: sprite.frame.h.into(),
                },
                &Rect {
                    x: 300.0,
                    y: 300.0,
                    width: sprite.frame.w.into(),
                    height: sprite.frame.h.into(),
                },
            );
        });
    }

    async fn initialize(&self) -> Result<Box<dyn Game>> {
        // TODO: into_serde is deprecated (presumably after book was written)
        let sheet = browser::fetch_json("rhb.json").await?.into_serde()?;
        let image = Some(engine::load_image("rhb.png").await?);

        Ok(Box::new(WalkTheDog {
            image,
            sheet,
            frame: self.frame,
        }))
    }

    fn update(&mut self) {}
}

#[derive(Deserialize)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Deserialize)]
struct Cell {
    frame: SheetRect,
}
