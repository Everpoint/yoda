use glium::glutin::event::{MouseButton, WindowEvent, MouseScrollDelta};
use crate::map::Map;
use crate::event::MapEvent;
use glium::backend::glutin::glutin::event::ElementState;
use glium::backend::glutin::glutin::dpi::PhysicalPosition;
use crate::Point;

#[derive(Debug, Default)]
pub struct MouseState {
    cursor_position: [i32; 2],
    left_button_pressed: bool,
    right_button_pressed: bool,
    middle_button_pressed: bool,
}

#[derive(Debug)]
pub struct ControlState {
    mouse_state: MouseState,
    pub map_size: [u32; 2],
    pub last_zoom_time: std::time::SystemTime,
}

impl Default for ControlState {
    fn default() -> Self {
        Self {
            mouse_state: MouseState::default(),
            map_size: [0, 0],
            last_zoom_time: std::time::SystemTime::now(),
        }
    }
}

pub struct MapControl<'a> {
    pub map: &'a mut Map,
    pub settings: MapControlSettings,
}

#[derive(Debug)]
pub struct MapControlSettings {
    mouse_wheel_speed: f32,
    zoom_delay: u32,
}

impl Default for MapControlSettings {
    fn default() -> Self {
        Self {
            mouse_wheel_speed: 2.0,
            zoom_delay: 50,
        }
    }
}

impl<'a> MapControl<'a> {
    pub fn handle_event(&mut self, event: &WindowEvent) {
        use WindowEvent::*;

        match event {
            MouseInput {button, state, ..} => {
                match state {
                    ElementState::Pressed => self.mouse_pressed(*button),
                    ElementState::Released => self.mouse_released(*button),
                }
            },
            CursorMoved {position, ..} => self.cursor_moved(position.x as i32, position.y as i32),
            MouseWheel {delta, ..} => self.wheel(*delta),
            _ => {},
        }
    }

    fn mouse_pressed(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.map.control_state_mut().mouse_state.left_button_pressed = true,
            MouseButton::Right => self.map.control_state_mut().mouse_state.right_button_pressed = true,
            MouseButton::Middle => self.map.control_state_mut().mouse_state.middle_button_pressed = true,
            MouseButton::Other(_) => {}
        }
    }

    fn mouse_released(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.map.control_state_mut().mouse_state.left_button_pressed = false,
            MouseButton::Right => self.map.control_state_mut().mouse_state.right_button_pressed = false,
            MouseButton::Middle => self.map.control_state_mut().mouse_state.middle_button_pressed = false,
            MouseButton::Other(_) => {}
        }
    }

    fn cursor_moved(&mut self, x: i32, y: i32) {
        let state = self.map.control_state();
        if state.mouse_state.left_button_pressed
            || state.mouse_state.right_button_pressed
            || state.mouse_state.middle_button_pressed {
            let prev_position = state.mouse_state.cursor_position;
            let dx = x - prev_position[0];
            let dy = y - prev_position[1];

            if state.mouse_state.left_button_pressed {
                self.map.trigger(&MapEvent::Drag {dx, dy: -dy});
            } else if state.mouse_state.right_button_pressed {
                self.map.trigger(&MapEvent::RightButtonDrag {dx, dy: -dy, cursor_position: state.mouse_state.cursor_position});
            } else if state.mouse_state.middle_button_pressed {
                self.map.trigger(&MapEvent::MiddleButtonDrag {dx, dy: -dy})
            }
        }

        self.map.control_state_mut().mouse_state.cursor_position = [x, y];
    }

    fn wheel(&mut self, delta: MouseScrollDelta) {
        if self.map.control_state().last_zoom_time.elapsed().unwrap().as_millis() < self.settings.zoom_delay as u128 {
            return;
        }

        let dy = match delta {
            MouseScrollDelta::LineDelta(_, dy) => dy,
            _ => 0.0,
        };

        const DELTA: f32 = 1.1;
        let delta = if dy > 0.0 {
            DELTA
        } else if dy < 0.0 {
            1.0 / DELTA
        } else {
            return;
        };

        let delta = delta.powf(self.settings.mouse_wheel_speed);
        self.map.trigger(&MapEvent::Zoom {delta, cursor_position: self.map.control_state().mouse_state.cursor_position});

        self.map.control_state_mut().last_zoom_time = std::time::SystemTime::now();
    }
}