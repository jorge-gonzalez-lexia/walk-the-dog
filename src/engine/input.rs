use crate::browser;
use anyhow::Result;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::JsCast;

pub enum KeyPress {
    KeyDown(web_sys::KeyboardEvent),
    KeyUp(web_sys::KeyboardEvent),
}

pub fn prepare_input() -> Result<UnboundedReceiver<KeyPress>> {
    let (keydown_sender, keyevent_receiver) = unbounded();
    let keydown_sender = Rc::new(RefCell::new(keydown_sender));
    let keyup_sender = Rc::clone(&keydown_sender);
    let onkeydown = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        if let Err(err) = keydown_sender
            .borrow_mut()
            .start_send(KeyPress::KeyDown(keycode))
        {
            error!("Error sending key down event {:#?}", err);
        }
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
    browser::window()?.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    onkeydown.forget();

    let onkeyup = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        if let Err(err) = keyup_sender
            .borrow_mut()
            .start_send(KeyPress::KeyUp(keycode))
        {
            error!("Error sending key up event {:#?}", err);
        }
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
    browser::window()?.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    onkeyup.forget();

    Ok(keyevent_receiver)
}

pub struct KeyState {
    pressed_keys: HashMap<String, web_sys::KeyboardEvent>,
}

impl KeyState {
    pub fn new() -> Self {
        KeyState {
            pressed_keys: HashMap::new(),
        }
    }

    pub fn is_pressed(&self, code: &str) -> bool {
        self.pressed_keys.contains_key(code)
    }

    pub fn set_pressed(&mut self, code: &str, event: web_sys::KeyboardEvent) {
        log!("Pressed {code}");
        self.pressed_keys.insert(code.into(), event);
    }

    fn set_released(&mut self, code: &str) {
        self.pressed_keys.remove(code);
    }
}

pub fn process_input(state: &mut KeyState, keyevent_receiver: &mut UnboundedReceiver<KeyPress>) {
    loop {
        match keyevent_receiver.try_next() {
            Ok(None) => break,
            Err(_err) => break,
            Ok(Some(evt)) => match evt {
                KeyPress::KeyDown(evt) => state.set_pressed(&evt.code(), evt),
                KeyPress::KeyUp(evt) => state.set_released(&evt.code()),
            },
        }
    }
}
