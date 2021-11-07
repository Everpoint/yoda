use std::rc::Rc;

use glow::{Context, HasContext};
use winit::event::{Event, StartCause::*, WindowEvent::*};
use winit::event_loop::ControlFlow;

use crate::map::Map;
use crate::render_target::RenderTarget;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

const WAIT_TIME: instant::Duration = instant::Duration::from_nanos(16_666_667);

fn event_loop_cycle(
    event: winit::event::Event<()>,
    control_flow: &mut winit::event_loop::ControlFlow,
    map: &mut Map,
    gl: Rc<Context>,
    width: u32,
    height: u32,
) -> bool {
    let mut redraw_requested = false;

    match event {
        Event::NewEvents(cause) => match cause {
            ResumeTimeReached { .. } => (),
            Init => (),
            Poll => (),
            _ => {}
        },
        Event::WindowEvent { event, .. } => match event {
            CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            Resized(size) => unsafe {
                gl.viewport(0, 0, size.width as i32, size.height as i32);
            },
            _ => {
                map.control().handle_event(&event);
            }
        },
        Event::MainEventsCleared => unsafe {
            gl.viewport(0, 0, width as i32, height as i32);
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            let mut target = RenderTarget::new(gl, (width, height));
            map.draw(&mut target);

            redraw_requested = true;
        },
        Event::RedrawEventsCleared => {
            *control_flow = ControlFlow::WaitUntil(instant::Instant::now() + WAIT_TIME);
        }
        _ => {}
    }

    redraw_requested
}
