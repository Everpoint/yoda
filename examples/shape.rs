use yoda::map::Map;
use yoda::layer::StaticLayer;
use yoda::render_target::RenderTarget;
use yoda::symbol::{CircleSymbol, LineSymbol, PolygonSymbol};
use winit::event_loop::{EventLoop, ControlFlow};
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};
use glow::HasContext;

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
    for i in 0..9 {
        for j in 0..9 {
            if i == 0 && j == 0 { continue; }

            for z in 0..len {
                let p = points[z];
                points.push([p[0] + i as f32 * width, p[1] + j as f32 * height, 0.0]);
                // points.push([p[0], p[1], j as f32 * 30000.0]);
            }
        }
    }

    eprintln!("Loaded {} points. ", points.len());

    let (event_loop, window) = init_window();

    let mut symbol = CircleSymbol { size: 5.0, color: [0.0, 0.7, 0.7, 1.0], program: None };
    let layer = StaticLayer::new(symbol, points);
    let mut map = Map::new();
    map.set_center((bbox[2] + bbox[0]) / 2.0, (bbox[3] + bbox[1]) / 2.0);
    map.set_resolution((bbox[2] - bbox[0]) / 800.0);

    map.add_layer(Box::new(layer));
    let mut last_frame_time = std::time::SystemTime::now();

    let gl = unsafe {
        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

        gl
    };

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::Resized(size) => {
                    unsafe {
                        gl.viewport(0, 0, size.width as i32, size.height as i32);
                    }
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
