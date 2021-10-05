use yoda::map::Map;
use yoda::layer::StaticLayer;
use yoda::render_target::RenderTarget;
use yoda::symbol::{CircleSymbol, LineSymbol, PolygonSymbol};
use winit::event_loop::{EventLoop, ControlFlow};
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};
use glow::HasContext;

fn main() {
    let (event_loop, window) = init_window();

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

    let gl = unsafe {
        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

        gl
    };

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
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

        let window_size = window.window().inner_size();
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            let mut target = RenderTarget::new(&gl, (window_size.width, window_size.height));
            map.draw(&mut target);

            window.swap_buffers();
        }
    });
}

fn init_window() -> (EventLoop<()>, ContextWrapper<PossiblyCurrent, Window>) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Simple yoda map example");
    let window = unsafe {
        glutin::ContextBuilder::new()
            .with_multisampling(4)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap()
    };

    (event_loop, window)
}