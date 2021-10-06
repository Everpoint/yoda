mod utils;
use winit::platform::web::WindowBuilderExtWebSys;
use glow::HasContext;

use wasm_bindgen::prelude::*;
use yoda::symbol::{CircleSymbol, LineSymbol, PolygonSymbol};
use yoda::layer::StaticLayer;
use yoda::map::Map;
use yoda::render_target::RenderTarget;
use winit::event_loop::ControlFlow;
use winit::dpi::{Size, PhysicalSize};
use winit::dpi::Size::Physical;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    console_error_panic_hook::set_once();

    use wasm_bindgen::JsCast;

    let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("map").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let context = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<web_sys::WebGl2RenderingContext>().unwrap();
    let gl = glow::Context::from_webgl2_context(context);

    let line_symbol = LineSymbol {width: 3.0, color: [0.5, 0.2, 0.0, 1.0], program: None};
    let line = vec![[0.0, 0.0, 0.0], [100.0, 100.0, 0.0], [100.0, 0.0, 0.0], [200.0, 0.0, 0.0], [200.0, 100.0, 0.0]];
    let line_layer = StaticLayer::new(line_symbol, vec![line]);

    let symbol = CircleSymbol { size: 20.0, color: [0.0, 0.7, 0.7, 1.0], program: None };
    let layer = StaticLayer::new(symbol, vec![[0.0, 0.0, 0.0], [100.0, 100.0, 0.0], [100.0, 0.0, 0.0], [0.0, 100.0, 0.0]]);

    let polygon_symbol = PolygonSymbol { fill_color: [0.0, 0.5, 0.3, 0.5], program: None};
    let polygon = vec![
        vec![
            [-150.0, -150.0, 0.0],
            [-150.0, 150.0, 0.0],
            [150.0, 150.0, 0.0],
            [150.0, -150.0, 0.0],
        ],
        vec![
            [-30.0, -30.0, 0.0],
            [30.0, -30.0, 0.0],
            [0.0, 30.0, 0.0],
        ]
    ];
    let polygon_layer = StaticLayer::new(polygon_symbol, vec![polygon]);

    let mut map = Map::new();
    map.add_layer(Box::new(line_layer));
    map.add_layer(Box::new(layer));
    map.add_layer(Box::new(polygon_layer));

    let width = web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;
    let height = web_sys::window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new().with_canvas(Some(canvas))
        .with_inner_size(PhysicalSize::new(width, height))
        .build(&event_loop).unwrap();
    // let window_size = window.inner_size();

    unsafe {
        gl.viewport(0, 0, width as i32, height as i32);
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
    }

    event_loop.run(move |event, _, control_flow| {
        // let next_frame_time = std::time::Instant::now() +
        //     std::time::Duration::from_nanos(16_666_667);
        let next_frame_time = instant::Instant::now() + instant::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                    return;
                },
                winit::event::WindowEvent::Resized(size) => {
                    // context.gl_window().resize(size);
                },
                _ => {
                    map.control().handle_event(&event)
                },
            },
            winit::event::Event::NewEvents(cause) => match cause {
                winit::event::StartCause::ResumeTimeReached { .. } => (),
                winit::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            let mut target = RenderTarget::new(&gl, (width, height));
            map.draw(&mut target);
        }
    });
}
