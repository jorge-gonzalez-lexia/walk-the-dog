use super::rect::{Point, Rect};
use anyhow::{anyhow, Result};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

const SHOW_BOUNDING_BOXES: bool = false;

pub struct Renderer {
    pub context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn clear(&self, rect: &Rect) {
        self.context.clear_rect(
            rect.x().into(),
            rect.y().into(),
            rect.width.into(),
            rect.height.into(),
        );
    }

    pub fn draw_entire_image(&self, image: &HtmlImageElement, position: &Point) {
        self.context
            .draw_image_with_html_image_element(image, position.x.into(), position.y.into())
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
    }

    /// Copy the given `frame` rectangle from the `image` and draw it on the
    /// canvas at the given `destination`
    pub fn draw_image(&self, image: &HtmlImageElement, frame: &Rect, destination: &Rect) {
        self.draw_image_ext(
            image,
            frame,
            destination,
            DrawImageOptions {
                flip_horizontally: false,
            },
        );
    }

    /// Copy the given `frame` rectangle from the `image` and draw it on the
    /// canvas at the given `destination`. Use Options to alter behavior.
    pub fn draw_image_ext(
        &self,
        image: &HtmlImageElement,
        frame: &Rect,
        destination: &Rect,
        options: DrawImageOptions,
    ) {
        if options.flip_horizontally {
            self.context.save();
            let tx = destination.x() + destination.width;
            // flip horizontally
            self.context
                .translate(tx.into(), 0.0)
                .expect("Error translating Canvas");
            self.context.scale(-1.0, 1.0).expect("Error scaling Canvas");
        }

        let dx = if options.flip_horizontally {
            0
        } else {
            destination.x()
        };

        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                frame.x().into(),
                frame.y().into(),
                frame.width.into(),
                frame.height.into(),
                dx.into(),
                destination.y().into(),
                destination.width.into(),
                destination.height.into(),
            )
            .expect("Drawing is throwing exceptions! Unrecoverable error.");

        if options.flip_horizontally {
            self.context.restore();
        }
    }

    pub fn draw_rect(&self, bounding_box: &Rect) {
        self.draw_rect_colored(bounding_box, "#FF0000");
    }

    pub fn draw_rect_colored(&self, bounding_box: &Rect, color: &str) {
        if !SHOW_BOUNDING_BOXES {
            return;
        }

        self.context.set_stroke_style(&JsValue::from_str(color));
        self.context.begin_path();
        self.context.rect(
            bounding_box.x().into(),
            bounding_box.y().into(),
            bounding_box.width.into(),
            bounding_box.height.into(),
        );
        self.context.stroke();
    }

    pub fn draw_text(&self, text: &str, location: &Point) -> Result<()> {
        self.context.set_font("16[t serif");
        self.context
            .fill_text(text, location.x.into(), location.y.into())
            .map_err(|err| anyhow!("Error filling text {:#?}", err))?;

        Ok(())
    }
}

pub struct DrawImageOptions {
    pub flip_horizontally: bool,
}
