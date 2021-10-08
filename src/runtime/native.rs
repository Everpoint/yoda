use crate::map::Map;
use glow::{Context, HasContext};
use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::window::Window;

pub struct NativeRuntime {
    map: Map,
    context: Context,
    window: ContextWrapper<PossiblyCurrent, Window>,
    event_loop: glutin::event_loop::EventLoop<()>,
}

impl NativeRuntime {
    pub fn new(f: &dyn Fn(glutin::window::WindowBuilder) -> glutin::window::WindowBuilder) -> Self {
        let event_loop = glutin::event_loop::EventLoop::new();

        let window_builder = f(glutin::window::WindowBuilder::new());
        let window = unsafe {
            glutin::ContextBuilder::new()
                .with_multisampling(4)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap()
        };

        let gl = unsafe {
            let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            gl
        };

        let map = Map::new();
        Self {
            map,
            context: gl,
            window,
            event_loop,
        }
    }

    pub fn map_mut(&mut self) -> &mut Map {
        &mut self.map
    }

    pub fn run(self) {
        let NativeRuntime {context, mut map, window, event_loop} = self;
        let gl = context;
        event_loop.run(move |event, _, control_flow| {
            let window_size = window.window().inner_size();
            super::event_loop_cycle(event, control_flow, &mut map, &gl, window_size.width, window_size.height);

            window.swap_buffers().unwrap();
        });
    }
}