use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, WheelEvent};

use crate::{matrix::Matrix3, App};

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
            let client_width = app.canvas.client_width() as f32;
            let client_height = app.canvas.client_height() as f32;
            // TODO: respect event.deltaMode (will currently scroll very slowly if not set to WheelEvent.DOM_DELTA_PIXEL)
            let delta_x = event.delta_x() as f32;
            let delta_y = event.delta_y() as f32;
            if app.viewport.keys_held.shift {
                // Pan only
                app.viewport.viewport_transform *= Matrix3::scale(
                    delta_x / client_width * 2.0 - 1.0,
                    delta_y / client_height * 2.0 - 1.0,
                );
            } else {
                // Zoom by applying affine transform
                let factor = 1.0
                    + if delta_x.abs() > delta_y.abs() {
                        delta_x / client_width
                    } else {
                        delta_y / client_height
                    };
                let Some((mouse_x, mouse_y)) =
                    app.viewport.pointers.get(0).map(|pointer| pointer.position)
                else {
                    return Ok(());
                };
                app.viewport.viewport_transform *=
                    Matrix3([1.0, 0.0, mouse_x, 0.0, 1.0, mouse_y, 0.0, 0.0, 1.0]);
                app.viewport.viewport_transform *=
                    Matrix3([factor, 0.0, 0.0, 0.0, factor, 0.0, 0.0, 0.0, 1.0]);
                app.viewport.viewport_transform *=
                    Matrix3([1.0, 0.0, -mouse_x, 0.0, 1.0, -mouse_y, 0.0, 0.0, 1.0]);
            }
            app.draw();
            Ok(())
        }
    }
}
