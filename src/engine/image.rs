use super::{
    rect::{Point, Rect},
    renderer::Renderer,
};
use crate::browser;
use anyhow::{anyhow, Result};
use futures::channel::oneshot::channel;
use std::{rc::Rc, sync::Mutex};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::HtmlImageElement;

pub struct Image {
    bounding_box: Rect,
    element: HtmlImageElement,
}

impl Image {
    pub fn new(element: HtmlImageElement, position: Point) -> Self {
        let bounding_box = Rect::new(position, element.width() as i16, element.height() as i16);
        Self {
            bounding_box,
            element,
        }
    }

    pub fn bounding_box(&self) -> &Rect {
        &self.bounding_box
    }

    pub fn draw(&self, renderer: &Renderer) {
        renderer.draw_entire_image(&self.element, &self.bounding_box.position);
        renderer.draw_rect(&self.bounding_box);
    }

    pub fn move_horizontally(&mut self, distance: i16) {
        self.set_x(self.bounding_box.x() + distance);
    }

    pub fn right(&self) -> i16 {
        self.bounding_box.right()
    }

    pub fn set_x(&mut self, x: i16) {
        self.bounding_box.set_x(x);
        self.bounding_box.position.x = x;
    }
}

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image()?;
    let (complete_tx, complete_rx) = channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);

    let success_callback = browser::closure_once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            success_tx.send(Ok(())).unwrap();
        }
    });

    let error_callback: Closure<dyn FnMut(JsValue)> = browser::closure_once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            error_tx
                .send(Err(anyhow!("Error Loading Image: {:#?}", err)))
                .unwrap();
        }
    });

    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);
    complete_rx.await??;

    Ok(image)
}
