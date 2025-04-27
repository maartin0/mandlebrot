extern crate console_error_panic_hook;
use std::{cell::RefCell, panic, rc::Rc, sync::LazyLock};

use fragile::Fragile;
use shader::{draw, init_shaders, CanvasState};
use util::{console_log, request_animation_frame, start_animation_loop};
use viewport::ViewportController;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::{
    window, HtmlCanvasElement, HtmlElement, Performance, WebGl2RenderingContext, Window,
};

mod arbitrary_num;
mod matrix;
mod shader;
mod util;
mod viewport;

pub const DEPTH: usize = 500;

#[derive(Clone, Debug)]
struct App {
    window: Window,
    body: HtmlElement,
    canvas: HtmlCanvasElement,
    context: WebGl2RenderingContext,
    performance: Performance,
    props: CanvasState,
    viewport: ViewportController,
    running_animations: u32,
}

fn dispatch_draw_callback(app_ref: &Rc<RefCell<App>>) -> Result<(), JsValue> {
    app_ref.borrow().draw();
    Ok(())
}

fn dispatch_animate(app_ref: &Rc<RefCell<App>>) -> Result<bool, JsValue> {
    let mut app = app_ref.borrow_mut();
    Ok(if app.running_animations > 0 {
        app.animate();
        true
    } else {
        false
    })
}

impl App {
    fn init() -> Result<Self, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id("root").unwrap().dyn_into()?;
        let context: WebGl2RenderingContext = canvas.get_context("webgl2")?.unwrap().dyn_into()?;
        let performance = window.performance().unwrap();

        let props = init_shaders(&context);

        let mut result = Self {
            window,
            body,
            canvas,
            context,
            performance,
            props,
            viewport: Default::default(),
            running_animations: 0,
        };
        result.resize();
        Ok(result)
    }

    fn dispatch_draw(app_ref: Rc<RefCell<App>>) -> Result<(), JsValue> {
        let window = app_ref.borrow().window.clone();
        request_animation_frame(app_ref, &window, dispatch_draw_callback)
    }

    fn draw(&self) {
        draw(&self.context, &self.props, self.viewport.transform())
    }

    fn setup(app_ref: &Rc<RefCell<App>>) -> Result<(), JsValue> {
        App::dispatch_draw(app_ref.clone())?;
        App::register_viewport_events(app_ref.clone())
    }

    pub fn start_animation(app_ref: Rc<RefCell<App>>) -> Result<(), JsValue> {
        if app_ref.borrow().running_animations > 0 {
            return Ok(());
        }
        let window = app_ref.borrow().window.clone();
        app_ref.borrow_mut().running_animations += 1;
        start_animation_loop(app_ref.clone(), &window, dispatch_animate)
    }

    pub fn stop_animation(&mut self) {
        self.running_animations = 0;
    }
}

static STATE: LazyLock<Fragile<Rc<RefCell<Option<Rc<RefCell<App>>>>>>> =
    LazyLock::new(|| Fragile::new(Rc::new(RefCell::new(None))));

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log!("Loading...");
    let app_ref = Rc::new(RefCell::new(App::init()?));
    *STATE.get().borrow_mut() = Some(app_ref.clone());
    App::setup(&app_ref)
}
