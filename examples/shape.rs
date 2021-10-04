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
use std::time::Duration;
use yoda::symbol::CircleSymbol;

fn main() {
    let shape_points = shapefile::reader::ShapeReader::from_path("./examples/data/points_wm.shp").unwrap().read().unwrap();
    let mut points = vec![];
    let mut bbox: [f32; 4] = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
    for shape in shape_points {
        match shape {
            shapefile::Shape::Point(p) => {
                points.push([p.x as f32, p.y as f32, 0.0]);
                bbox[0] = bbox[0].min(p.x as f32);
                bbox[1] = bbox[1].min(p.y as f32);
                bbox[2] = bbox[2].max(p.x as f32);
                bbox[3] = bbox[3].max(p.y as f32);
            },
            _ => {},
        }
    }

    let width = bbox[2] - bbox[0];
    let height = bbox[3] - bbox[1];
    let len = points.len();
    for i in 0..1 {
        for j in 0..3 {
            if i == 0 && j == 0 { continue; }

            for z in 0..len {
                let p = points[z];
                // points.push([p[0] + i as f32 * width, p[1] + j as f32 * height]);
                points.push([p[0], p[1], j as f32 * 30000.0]);
            }
        }
    }

    eprintln!("Loaded {} points. ", points.len());

    let (event_loop, context) = init_window();

    let mut symbol = CircleSymbol { size: 3.0, color: [0.0, 0.7, 0.7, 1.0], program: None };
    let layer = StaticLayer::new(symbol, points);
    let mut map = Map::new();
    map.set_center((bbox[2] + bbox[0]) / 2.0, (bbox[3] + bbox[1]) / 2.0);
    map.set_resolution((bbox[2] - bbox[0]) / 800.0);

    map.add_layer(Box::new(layer));
    let mut last_frame_time = std::time::SystemTime::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::Resized(size) => {
                    context.gl_window().resize(size);
                    return;
                },
                _ => {
                    map.control().handle_event(&event);
                    return;
                },
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut frame = context.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);

        let mut target = RenderTarget::new(&context, &mut frame);
        map.draw(&mut target);

        frame.finish().unwrap();

        last_frame_time = std::time::SystemTime::now();
    });
}

fn init_window() -> (EventLoop<()>, Display) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_multisampling(4);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();;

    (event_loop, display)
}