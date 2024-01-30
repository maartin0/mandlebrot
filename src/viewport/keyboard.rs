use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, KeyboardEvent};

use crate::App;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct KeysHeld {
    pub shift: bool,
    pub ctrl: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub plus: bool,
    pub minus: bool,
}

impl App {
    pub fn key_down_listener(app_ref: Rc<RefCell<App>>) -> impl Fn(Event) -> Result<(), JsValue> {
        move |event_raw| -> Result<(), JsValue> {
            let event: KeyboardEvent = event_raw.dyn_into().unwrap();
            {
                let keys_held = &mut app_ref.borrow_mut().viewport.keys_held;
                match event.key().as_str() {
                    "ArrowUp" => keys_held.up = true,
                    "ArrowDown" => keys_held.down = true,
                    "ArrowLeft" => keys_held.left = true,
                    "ArrowRight" => keys_held.right = true,
                    "=" | "+" => keys_held.plus = true,
                    "-" | "_" => keys_held.minus = true,
                    "Shift" => keys_held.shift = true,
                    "Control" => keys_held.ctrl = true,
                    _ => {}
                }
            }
            App::start_animation(app_ref.clone())?;
            Ok(())
        }
    }

    pub fn key_up_listener(app_ref: Rc<RefCell<App>>) -> impl Fn(Event) -> Result<(), JsValue> {
        move |event_raw| -> Result<(), JsValue> {
            let event: KeyboardEvent = event_raw.dyn_into().unwrap();
            let mut app = app_ref.borrow_mut();
            let keys_held = &mut app.viewport.keys_held;
            match event.key().as_str() {
                "ArrowUp" => keys_held.up = false,
                "ArrowDown" => keys_held.down = false,
                "ArrowLeft" => keys_held.left = false,
                "ArrowRight" => keys_held.right = false,
                "=" | "+" => keys_held.plus = false,
                "-" | "_" => keys_held.minus = false,
                "Shift" => keys_held.shift = false,
                "Control" => keys_held.ctrl = false,
                _ => {}
            }
            app.stop_animation();
            Ok(())
        }
    }
}
