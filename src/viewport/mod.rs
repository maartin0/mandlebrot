use std::{cell::RefCell, rc::Rc};

use keyboard::KeysHeld;
use pointer::Pointer;
use wasm_bindgen::JsValue;
use web_sys::Event;

use crate::{matrix::Matrix3, util::add_event_listener, App};

mod keyboard;
mod pointer;
mod wheel;
mod window;

#[derive(Debug, Clone, Default)]
pub struct ViewportController {
    keys_held: KeysHeld,
    pointers: Vec<Pointer>,
    last_frame_ms: f32,
    viewport_transform: Matrix3,
    window_transform: Matrix3,
}

impl ViewportController {
    pub fn transform(&self) -> Matrix3 {
        self.viewport_transform * self.window_transform
    }
}

impl App {
    pub fn release_all_listener(
        app_ref: Rc<RefCell<App>>,
    ) -> impl FnMut(Event) -> Result<(), JsValue> {
        move |_| {
            let mut app = app_ref.borrow_mut();
            app.viewport.keys_held = Default::default();
            app.viewport.pointers = Vec::new();
            app.stop_animation();
            Ok(())
        }
    }
}

impl App {
    pub fn register_viewport_events(app_ref: Rc<RefCell<App>>) -> Result<(), JsValue> {
        let window = app_ref.borrow().window.clone();
        let parent = app_ref.borrow().canvas.clone();
        {
            add_event_listener(
                &parent,
                "wheel",
                App::wheel_listener(app_ref.clone()),
                false,
            )?;
            add_event_listener(
                &parent,
                "pointermove",
                App::pointer_move_listener(app_ref.clone()),
                true,
            )?;
            add_event_listener(
                &parent,
                "pointerleave",
                App::release_all_listener(app_ref.clone()),
                true,
            )?;
            add_event_listener(
                &parent,
                "pointercancel",
                App::release_all_listener(app_ref.clone()),
                true,
            )?;
            add_event_listener(
                &parent,
                "pointerup",
                App::pointer_up_listener(app_ref.clone()),
                true,
            )?;
            add_event_listener(
                &parent,
                "pointerdown",
                App::pointer_down_listener(app_ref.clone()),
                true,
            )?;
            add_event_listener(
                &window,
                "focusin",
                App::release_all_listener(app_ref.clone()),
                true,
            )?;
            add_event_listener(
                &window,
                "focusout",
                App::release_all_listener(app_ref.clone()),
                true,
            )?;
            add_event_listener(
                &window,
                "keydown",
                App::key_down_listener(app_ref.clone()),
                true,
            )?;
            add_event_listener(
                &window,
                "keyup",
                App::key_up_listener(app_ref.clone()),
                true,
            )?;
            add_event_listener(
                &window,
                "resize",
                App::resize_listener(app_ref.clone()),
                true,
            )?;
        }
        Ok(())
    }

    pub fn resize(&mut self) {
        let width = self.body.client_width();
        let height = self.body.client_height();

        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);

        self.context.viewport(0, 0, width, height);

        // Avoid stretching viewport
        self.viewport.window_transform = Matrix3::scale(1.0, height as f32 / width as f32);
    }
}
