use winit::dpi::PhysicalSize;
use winit::event_loop::ControlFlow;
use crate::render_target::RenderTarget;
use crate::map::Map;
use glow::{Context, HasContext};
use winit::window::Window;
use winit::platform::web::WindowBuilderExtWebSys;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use crate::control::DefaultMapControl;
use std::cell::RefCell;

pub struct WasmRuntime {
    map: Map,
    control: DefaultMapControl,
    context: Context,
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
}

impl WasmRuntime {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Self {
        let context = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<web_sys::WebGl2RenderingContext>().unwrap();
        let gl = glow::Context::from_webgl2_context(context);

        let width = web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;
        let height = web_sys::window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;

        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new().with_canvas(Some(canvas))
            .with_inner_size(PhysicalSize::new(width, height))
            .build(&event_loop).unwrap();

        unsafe {
            gl.viewport(0, 0, width as i32, height as i32);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        }

        let map = Map::new();
        let control = DefaultMapControl::new();
        Self {
            map,
            window,
            context: gl,
            event_loop,
            control,
        }
    }

    pub fn map_mut(&mut self) -> &mut Map {
        &mut self.map
    }

    pub fn run(self) {
        let WasmRuntime {mut map, mut control, context, event_loop, window} = self;
        let gl = Rc::new(context);
        let map = Rc::new(RefCell::new(map));
        control.attach(map.clone());


        event_loop.run(move |event, _, control_flow| {
            let size = window.inner_size();
            super::event_loop_cycle(event, control_flow, &mut *map.borrow_mut(), gl.clone(), size.width, size.height);
        });
    }
}