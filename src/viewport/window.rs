use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::JsValue;
use web_sys::Event;

use crate::{matrix::Matrix3, App};

impl App {
    pub fn resize_listener(app_ref: Rc<RefCell<App>>) -> impl Fn(Event) -> Result<(), JsValue> {
        move |_| -> Result<(), JsValue> {
            let mut app = app_ref.borrow_mut();
            app.resize();
            app.draw();
            Ok(())
        }
    }

    pub fn animate(&mut self) {
        let now = self.performance.now() as f32;
        let mul = 100_f32.min(now as f32 - self.viewport.last_frame_ms) / 500.0;
        self.viewport.last_frame_ms = now;
        let mut state_changed = false;
        if self.viewport.keys_held.plus {
            // zoom in
            let m13 = self.viewport.viewport_transform[(0, 2)];
            let m23 = self.viewport.viewport_transform[(1, 2)];
            self.viewport.viewport_transform *=
                Matrix3([1.0, 0.0, m13, 0.0, 1.0, m23, 0.0, 0.0, 1.0]);
            self.viewport.viewport_transform *=
                Matrix3([1.0 - mul, 0.0, 0.0, 0.0, 1.0 - mul, 0.0, 0.0, 0.0, 1.0]);
            self.viewport.viewport_transform *=
                Matrix3([1.0, 0.0, -m13, 0.0, 1.0, -m23, 0.0, 0.0, 1.0]);
            state_changed = true;
        }
        if self.viewport.keys_held.minus {
            // zoom out
            let m13 = self.viewport.viewport_transform[(0, 2)];
            let m23 = self.viewport.viewport_transform[(1, 2)];
            self.viewport.viewport_transform *=
                Matrix3([1.0, 0.0, m13, 0.0, 1.0, m23, 0.0, 0.0, 1.0]);
            self.viewport.viewport_transform *=
                Matrix3([1.0 + mul, 0.0, 0.0, 0.0, 1.0 + mul, 0.0, 0.0, 0.0, 1.0]);
            self.viewport.viewport_transform *=
                Matrix3([1.0, 0.0, -m13, 0.0, 1.0, -m23, 0.0, 0.0, 1.0]);
            state_changed = true;
        }
        if self.viewport.keys_held.left {
            // pan left
            self.viewport.viewport_transform *=
                Matrix3([1.0, 0.0, -mul, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
            state_changed = true;
        }
        if self.viewport.keys_held.right {
            // pan right
            self.viewport.viewport_transform *=
                Matrix3([1.0, 0.0, mul, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
            state_changed = true;
        }
        if self.viewport.keys_held.up {
            // pan up
            self.viewport.viewport_transform *=
                Matrix3([1.0, 0.0, 0.0, 0.0, 1.0, mul, 0.0, 0.0, 1.0]);
            state_changed = true;
        }
        if self.viewport.keys_held.down {
            // pan down
            self.viewport.viewport_transform *=
                Matrix3([1.0, 0.0, 0.0, 0.0, 1.0, -mul, 0.0, 0.0, 1.0]);
            state_changed = true;
        }
        if state_changed {
            self.draw();
        }
    }
}
