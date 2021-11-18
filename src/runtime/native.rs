use std::cell::RefCell;
use std::rc::Rc;

use glow::{Context, HasContext};
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};

use crate::control::DefaultMapControl;
use crate::map::Map;

pub struct NativeRuntime {
    map: Map,
    context: Rc<Context>,
    window_context: ContextWrapper<PossiblyCurrent, Window>,
    event_loop: glutin::event_loop::EventLoop<()>,
    control: DefaultMapControl,
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
            let gl =
                glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            gl
        };

        let map = Map::new();
        let control = DefaultMapControl::new();

        Self {
            map,
            context: Rc::new(gl),
            window_context: window,
            event_loop,
            control,
        }
    }

    pub fn map_mut(&mut self) -> &mut Map {
        &mut self.map
    }

    pub fn context(&self) -> Rc<Context> {
        self.context.clone()
    }

    pub fn run(self) {
        let NativeRuntime {
            context,
            map,
            window_context,
            event_loop,
            mut control,
        } = self;
        let map = Rc::new(RefCell::new(map));
        control.attach(map.clone());

        event_loop.run(move |event, _, control_flow| {
            let size = window_context.window().inner_size();

            let redraw_requested = super::event_loop_cycle(
                event,
                control_flow,
                &mut *map.borrow_mut(),
                context.clone(),
                size.width,
                size.height,
            );

            if redraw_requested {
                window_context.swap_buffers().unwrap();
            }
        });
    }
}
