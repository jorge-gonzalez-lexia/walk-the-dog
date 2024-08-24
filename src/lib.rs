#[macro_use]
mod browser;
mod engine;
mod game;

use anyhow::{anyhow, Result};
use rand::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let context = browser::context().expect("Could not get browser context");

    browser::spawn_local(async move {
        // TODO: into_serde is deprecated (presumably after book was written)
        let sheet: Sheet = browser::fetch_json("rhb.json")
            .await
            .expect("Could not fetch rhb.json")
            .into_serde()
            .expect("Could not convert rhb.json into a Sheet structure");

        let image = engine::load_image("rhb.png")
            .await
            .expect("Could not load rhb.png");

        let mut frame = -1;
        let interval_callback = Closure::wrap(Box::new(move || {
            frame = (frame + 1) % 8;
            let frame_name = format!("Run ({}).png", frame + 1);
            let sprite = sheet.frames.get(&frame_name).expect(&frame_name);
            // .or_else(|| panic!("Cell '{}' not found", frame_name));

            context.clear_rect(0.0, 0.0, 600.0, 600.0);
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                sprite.frame.x.into(),
                sprite.frame.y.into(),
                sprite.frame.w.into(),
                sprite.frame.h.into(),
                300.0,
                300.0,
                sprite.frame.w.into(),
                sprite.frame.h.into(),
            );
        }) as Box<dyn FnMut()>);
        browser::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                interval_callback.as_ref().unchecked_ref(),
                50,
            );
        interval_callback.forget();
    });

    Ok(())
}

fn sierpinski(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    color: (u8, u8, u8),
    depth: u8,
) {
    draw_triangle(&context, points, color);
    let depth = depth - 1;
    if depth > 0 {
        let mut rnd = thread_rng();
        let next_color = (
            rnd.gen_range(0..255),
            rnd.gen_range(0..255),
            rnd.gen_range(0..255),
        );
        let [top, left, right] = points;
        let left_middle = midpoint(top, left);
        let right_middle = midpoint(top, right);
        let bottom_middle = midpoint(left, right);
        sierpinski(
            &context,
            [top, left_middle, right_middle],
            next_color,
            depth,
        );
        sierpinski(
            &context,
            [left_middle, left, bottom_middle],
            next_color,
            depth,
        );
        sierpinski(
            &context,
            [right_middle, bottom_middle, right],
            next_color,
            depth,
        );
    }
}

fn draw_triangle(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    color: (u8, u8, u8),
) {
    let color_str = format!("rgb({},{},{})", color.0, color.1, color.2);
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));

    let [top, left, right] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();

    context.stroke();
    context.fill();
}

fn midpoint(point_1: (f64, f64), point_2: (f64, f64)) -> (f64, f64) {
    ((point_1.0 + point_2.0) / 2.0, (point_1.1 + point_2.1) / 2.0)
}
