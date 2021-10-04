#[allow(unused_imports)]
use glium::{glutin, Surface};

#[macro_use]
extern crate glium;

use glium::backend::glutin::glutin::event_loop::EventLoop;
use glium::Display;
use glium::backend::glutin::glutin::event::ElementState;
use yoda::map::Map;
use yoda::layer::StaticLayer;
use yoda::render_target::RenderTarget;
use yoda::symbol::{CircleSymbol, LineSymbol, PolygonSymbol};

fn main() {
    let (event_loop, context) = init_window();

    let line_symbol = LineSymbol {width: 3.0, color: [0.5, 0.2, 0.0, 1.0], program: None};
    let line = vec![[0.0, 0.0, 0.0], [100.0, 100.0, 0.0], [100.0, 0.0, 0.0]];
    let line_layer = StaticLayer::new(line_symbol, vec![line]);

    let symbol = CircleSymbol { size: 10.0, color: [0.0, 0.7, 0.7, 1.0], program: None };
    let layer = StaticLayer::new(symbol, vec![[0.0, 0.0, 0.0], [100.0, 100.0, 0.0], [100.0, 0.0, 0.0]]);

    let polygon_symbol = PolygonSymbol {color: [0.0, 0.5, 0.3, 0.5], program: None};
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

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::Resized(size) => {
                    context.gl_window().resize(size);
                },
                _ => {
                    map.control().handle_event(&event)
                },
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut frame = context.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);

        let mut target = RenderTarget::new(&context, &mut frame);
        map.draw(&mut target);

        frame.finish().unwrap();
    });
}

fn init_window() -> (EventLoop<()>, Display) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_multisampling(4);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();;

    (event_loop, display)
}