use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, PointerEvent};

use crate::{arbitrary_num::ArbitaryNum, matrix::Matrix3, App};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Pointer {
    pub id: i32,
    pub position: (f32, f32),
    pub down_position: Option<(f32, f32)>,
}

impl App {
    /// Initialises a pointer if not already, returning a (delta_x, delta_y) pair of changed coordinates
    fn init_pointer<'a>(
        &'a mut self,
        event: &PointerEvent,
        going_down: bool,
        going_up: bool,
    ) -> (&'a mut Pointer, (f32, f32)) {
        let id = event.pointer_id();
        let new_pos = self.extract_pos(event);
        for (index, pointer) in self.viewport.pointers.iter().enumerate() {
            if pointer.id == id {
                let delta = (
                    new_pos.0 - pointer.position.0,
                    new_pos.1 - pointer.position.1,
                );
                let pointer_mut = self.viewport.pointers.get_mut(index).unwrap();
                pointer_mut.position = new_pos;
                if going_down {
                    pointer_mut.down_position = Some(new_pos);
                } else if going_up {
                    pointer_mut.down_position = None;
                }
                return (pointer_mut, delta);
            }
        }
        self.viewport.pointers.push(Pointer {
            id,
            position: new_pos,
            down_position: if going_down { Some(new_pos) } else { None },
        });
        (self.viewport.pointers.last_mut().unwrap(), (0.0, 0.0))
    }

    fn extract_pos(&mut self, event: &PointerEvent) -> (f32, f32) {
        let rect = self.canvas.get_bounding_client_rect();
        (
            (event.client_x() as f32 - rect.left() as f32) / self.canvas.client_width() as f32
                * 2.0
                - 1.0,
            (event.client_y() as f32 - rect.top() as f32) / self.canvas.client_height() as f32
                * 2.0
                - 1.0,
        )
    }

    pub fn pointer_move_listener(
        app_ref: Rc<RefCell<App>>,
    ) -> impl Fn(Event) -> Result<(), JsValue> {
        move |event_raw| -> Result<(), JsValue> {
            let event: PointerEvent = event_raw.dyn_into().unwrap();
            let mut app = app_ref.borrow_mut();
            let (pointer, (delta_x, delta_y)) = app.init_pointer(&event, false, false);
            if let Some((pointer_x, pointer_y)) = pointer.down_position {
                if app.viewport.pointers.len() == 1 {
                    // Pan
                    app.viewport.viewport_transform *=
                        Matrix3::translate(ArbitaryNum::from(-delta_x), ArbitaryNum::from(delta_y));
                } else {
                    // Touchscreen: scale around midpoint by applying affine transform
                    let (xs, ys): (Vec<f32>, Vec<f32>) = app
                        .viewport
                        .pointers
                        .iter()
                        .filter_map(|pointer| pointer.down_position)
                        .unzip();
                    let midpoint_x = xs.iter().sum::<f32>() / xs.len() as f32;
                    let midpoint_y = ys.iter().sum::<f32>() / ys.len() as f32;
                    let i = delta_x * (pointer_x - midpoint_x).signum();
                    let j = delta_y * (pointer_y - midpoint_y).signum();
                    let factor =
                        1.0 - (i.powf(2.0) + j.powf(2.0)).powf(0.5) * i.signum() * j.signum();
                    app.viewport.viewport_transform *= Matrix3::translate(
                        ArbitaryNum::from(midpoint_x),
                        ArbitaryNum::from(midpoint_y),
                    );
                    app.viewport.viewport_transform *=
                        Matrix3::scale(ArbitaryNum::from(factor), ArbitaryNum::from(factor));
                    app.viewport.viewport_transform *= Matrix3::translate(
                        ArbitaryNum::from(-midpoint_x),
                        ArbitaryNum::from(-midpoint_y),
                    );
                }
                app.draw();
            }
            Ok(())
        }
    }

    pub fn pointer_down_listener<'a>(
        app_ref: Rc<RefCell<App>>,
    ) -> impl Fn(Event) -> Result<(), JsValue> + use<'a> {
        move |event_raw| -> Result<(), JsValue> {
            let event: PointerEvent = event_raw.dyn_into().unwrap();
            app_ref.borrow_mut().init_pointer(&event, true, false);
            Ok(())
        }
    }

    pub fn pointer_up_listener<'a>(
        app_ref: Rc<RefCell<App>>,
    ) -> impl Fn(Event) -> Result<(), JsValue> + use<'a> {
        move |event_raw| -> Result<(), JsValue> {
            let event: PointerEvent = event_raw.dyn_into().unwrap();
            let mut app = app_ref.borrow_mut();
            app.init_pointer(&event, false, true);
            Ok(())
        }
    }
}
