use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::JsValue;
use web_sys::Event;

use crate::{arbitrary_num::ArbitaryNum, matrix::Matrix3, App};

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
        let now = self.performance.now();
        let mul = ArbitaryNum::from(100).min(ArbitaryNum::from(now - self.viewport.last_frame_ms))
            / ArbitaryNum::from(500);
        self.viewport.last_frame_ms = now;
        let mut state_changed = false;
        if self.viewport.keys_held.plus {
            // zoom in
            let m13 = self.viewport.viewport_transform[(0, 2)].clone();
            let m23 = self.viewport.viewport_transform[(1, 2)].clone();
            self.viewport.viewport_transform *= Matrix3::translate(m13.clone(), m23.clone());
            self.viewport.viewport_transform *= Matrix3::scale(
                ArbitaryNum::one() - mul.clone(),
                ArbitaryNum::one() - mul.clone(),
            );
            self.viewport.viewport_transform *= Matrix3::translate(-m13.clone(), -m23.clone());
            state_changed = true;
        }
        if self.viewport.keys_held.minus {
            // zoom out
            let m13 = self.viewport.viewport_transform[(0, 2)].clone();
            let m23 = self.viewport.viewport_transform[(1, 2)].clone();
            self.viewport.viewport_transform *= Matrix3::translate(m13.clone(), m23.clone());
            self.viewport.viewport_transform *= Matrix3::scale(
                ArbitaryNum::one() + mul.clone(),
                ArbitaryNum::one() + mul.clone(),
            );
            self.viewport.viewport_transform *= Matrix3::translate(-m13.clone(), -m23.clone());
            state_changed = true;
        }
        if self.viewport.keys_held.left {
            // pan left
            self.viewport.viewport_transform *=
                Matrix3::translate(-mul.clone(), ArbitaryNum::zero());
            state_changed = true;
        }
        if self.viewport.keys_held.right {
            // pan right
            self.viewport.viewport_transform *=
                Matrix3::translate(mul.clone(), ArbitaryNum::zero());
            state_changed = true;
        }
        if self.viewport.keys_held.up {
            // pan up
            self.viewport.viewport_transform *=
                Matrix3::translate(ArbitaryNum::zero(), mul.clone());
            state_changed = true;
        }
        if self.viewport.keys_held.down {
            // pan down
            self.viewport.viewport_transform *=
                Matrix3::translate(ArbitaryNum::zero(), -mul.clone());
            state_changed = true;
        }
        if state_changed {
            self.draw();
        }
    }
}
