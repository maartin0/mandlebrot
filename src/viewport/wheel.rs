use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, WheelEvent};

use crate::{arbitrary_num::ArbitaryNum, matrix::Matrix3, App};

impl App {
    pub fn wheel_listener(app_ref: Rc<RefCell<App>>) -> impl Fn(Event) -> Result<(), JsValue> {
        move |event_raw| -> Result<(), JsValue> {
            let mut app = app_ref.borrow_mut();
            let event: WheelEvent = event_raw.dyn_into().unwrap();
            if app.viewport.keys_held.ctrl {
                return Ok(());
            }
            event.prevent_default();
            event.stop_propagation();
            let client_width = ArbitaryNum::from(app.canvas.client_width());
            let client_height = ArbitaryNum::from(app.canvas.client_height());
            // TODO: respect event.deltaMode (will currently scroll very slowly if not set to WheelEvent.DOM_DELTA_PIXEL)
            let delta_x = ArbitaryNum::from(event.delta_x());
            let delta_y = ArbitaryNum::from(event.delta_y());
            if app.viewport.keys_held.shift {
                // Pan only
                app.viewport.viewport_transform *= Matrix3::scale(
                    delta_x / client_width * ArbitaryNum::two() - ArbitaryNum::one(),
                    delta_y / client_height * ArbitaryNum::two() - ArbitaryNum::one(),
                );
            } else {
                // Zoom by applying affine transform
                let factor = ArbitaryNum::one()
                    + if delta_x.clone().abs() > delta_y.clone().abs() {
                        delta_x / client_width
                    } else {
                        delta_y / client_height
                    };
                let Some((mouse_x, mouse_y)) = app.viewport.pointers.get(0).map(|pointer| {
                    (
                        ArbitaryNum::from(pointer.position.0),
                        ArbitaryNum::from(pointer.position.1),
                    )
                }) else {
                    return Ok(());
                };
                app.viewport.viewport_transform *=
                    Matrix3::translate(mouse_x.clone(), mouse_y.clone());
                app.viewport.viewport_transform *= Matrix3::scale(factor.clone(), factor);
                app.viewport.viewport_transform *= Matrix3::translate(-mouse_x, -mouse_y);
            }
            app.draw();
            Ok(())
        }
    }
}
