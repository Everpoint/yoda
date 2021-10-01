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

fn main() {
    let (event_loop, context) = init_window();

    let layer = StaticLayer::new(vec![[0.0, 0.0], [100.0, 100.0], [100.0, 0.0]]);
    let mut map = Map::new();
    map.add_layer(Box::new(layer));

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