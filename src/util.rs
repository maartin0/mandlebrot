use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{console, AddEventListenerOptions, Event, EventTarget, Window};

pub fn log(value: &str) {
    console::log_1(&JsValue::from_str(value))
}

#[allow(unused)]
macro_rules! console_log {
    ($($t:tt)*) => ($crate::util::log(&format_args!($($t)*).to_string()))
}

#[allow(unused)]
pub(crate) use console_log;

#[allow(unused)]
macro_rules! console_debug {
    ($inner:expr) => {{
        let result = $inner;
        $crate::util::console_log!("{:?} = {:?}", stringify!($inner), result);
        result
    }};
    ($($inner:expr),*) => {
        $($crate::util::console_debug!($inner));*
    }
}

#[allow(unused)]
pub(crate) use console_debug;

/// Requests an animation frame with the provided callback and arguments
pub fn request_animation_frame<Args: 'static>(
    args: Args,
    window: &Window,
    callback: impl Fn(&Args) -> Result<(), JsValue> + 'static,
) -> Result<(), JsValue> {
    let closure =
        Closure::<dyn Fn() -> Result<(), JsValue>>::new(move || -> Result<(), JsValue> {
            callback(&args)
        });
    window.request_animation_frame(closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}

/// Starts an animation loop with the provided callback and arguments
///
/// If the callback returns false, the loop will terminate
pub fn start_animation_loop<Args: 'static>(
    args: Args,
    window: &Window,
    callback: impl Fn(&Args) -> Result<bool, JsValue> + 'static,
) -> Result<(), JsValue> {
    let f: Rc<RefCell<Option<Closure<dyn Fn() -> Result<(), JsValue>>>>> =
        Rc::new(RefCell::new(None));
    let g = f.clone();
    let window_copy = window.clone();
    *g.borrow_mut() = Some(Closure::<dyn Fn() -> Result<(), JsValue>>::new(
        move || -> Result<(), JsValue> {
            if callback(&args)? {
                window_copy.request_animation_frame(
                    f.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
                )?;
            }
            Ok(())
        },
    ));
    window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;
    Ok(())
}

/// Adds the provided Rust closure as an event listener, returning the built [`JsValue`] closure which can be used to de-register the event
pub fn add_event_listener(
    target: &EventTarget,
    ty: &str,
    listener: impl FnMut(Event) -> Result<(), JsValue> + 'static,
    passive: bool,
) -> Result<JsValue, JsValue> {
    let options = AddEventListenerOptions::new();
    options.set_passive(passive);
    let value = Closure::<dyn FnMut(Event) -> Result<(), JsValue>>::new(listener).into_js_value();
    target.add_event_listener_with_callback_and_add_event_listener_options(
        ty,
        value.unchecked_ref(),
        &options,
    )?;
    Ok(value)
}
