use crate::map::Map;
use glow::{Context, HasContext};
use crate::render_target::RenderTarget;
use std::rc::Rc;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

fn event_loop_cycle(event: winit::event::Event<()>, control_flow: &mut winit::event_loop::ControlFlow, map: &mut Map, gl: Rc<Context>, width: u32, height: u32) {
    match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
                return;
            },
            winit::event::WindowEvent::Resized(size) => {
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
        winit::event::Event::NewEvents(cause) => match cause {
            winit::event::StartCause::ResumeTimeReached { .. } => (),
            winit::event::StartCause::Init => (),
            winit::event::StartCause::Poll => (),
            _ => return,
        },
        _ => return,
    }

    let next_frame_time = instant::Instant::now() + instant::Duration::from_nanos(16_666_667);
    *control_flow = winit::event_loop::ControlFlow::WaitUntil(next_frame_time);

    unsafe {
        gl.viewport(0, 0, width as i32, height as i32);
        gl.clear_color(0.1, 0.2, 0.3, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);

        let mut target = RenderTarget::new(gl, (width, height));
        map.draw(&mut target);
    }
}