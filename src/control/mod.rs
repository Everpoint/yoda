mod default;
pub use default::DefaultMapControl;

use crate::map::Map;
use crate::event::{ClickEvent, HandlerStore, TypedHandlerStore, EventListener, DragEvent, ZoomEvent};
use winit::event::{WindowEvent, ElementState, MouseButton, MouseScrollDelta};

#[derive(Debug)]
pub struct MouseState {
    cursor_position: [i32; 2],
    left_button_down_position: [i32; 2],
    middle_button_down_position: [i32; 2],
    right_button_down_position: [i32; 2],
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            cursor_position: [0, 0],
            left_button_down_position: [i32::MIN, i32::MIN],
            middle_button_down_position: [i32::MIN, i32::MIN],
            right_button_down_position: [i32::MIN, i32::MIN],
        }
    }
}

impl MouseState {
    fn capture_button_pressed(&mut self, button: MouseButton) {
        self.capture_button_position(button, self.cursor_position);
    }

    fn capture_button_released(&mut self, button: MouseButton) {
        self.capture_button_position(button, [i32::MIN, i32::MIN]);
    }

    fn capture_button_position(&mut self, button: MouseButton, val: [i32; 2]) {
        match button {
            MouseButton::Left => self.left_button_down_position = val,
            MouseButton::Right => self.right_button_down_position = val,
            MouseButton::Middle => self.middle_button_down_position = val,
            MouseButton::Other(_) => {},
        }
    }

    fn button_pressed_position(&self, button: MouseButton) -> [i32; 2] {
        match button {
            MouseButton::Left => self.left_button_down_position,
            MouseButton::Right => self.right_button_down_position,
            MouseButton::Middle => self.middle_button_down_position,
            MouseButton::Other(_) => [i32::MIN, i32::MIN],
        }
    }

    fn button_pressed(&self, button: MouseButton) -> bool {
        self.button_pressed_position(button)[0] != i32::MIN
    }

    fn any_button_pressed(&self) -> bool {
        self.button_pressed(MouseButton::Left) || self.button_pressed(MouseButton::Middle) || self.button_pressed(MouseButton::Right)
    }
}

#[derive(Debug)]
pub struct ControlState {
    mouse_state: MouseState,
    pub map_size: [u32; 2],
    pub last_zoom_time: instant::Instant,
}

impl Default for ControlState {
    fn default() -> Self {
        Self {
            mouse_state: MouseState::default(),
            map_size: [0, 0],
            last_zoom_time: instant::Instant::now(),
        }
    }
}

pub struct MapEventDispatcher<'a> {
    pub map: &'a mut Map,
    pub settings: MapControlSettings,
}

#[derive(Debug)]
pub struct MapControlSettings {
    mouse_wheel_speed: f32,
    zoom_delay: u32,
    max_click_displacement: i32,
}

impl Default for MapControlSettings {
    fn default() -> Self {
        Self {
            mouse_wheel_speed: 2.0,
            zoom_delay: 50,
            max_click_displacement: 3,
        }
    }
}

impl<'a> MapEventDispatcher<'a> {
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
        self.map.control_state_mut().mouse_state.capture_button_pressed(button);
    }

    fn mouse_released(&mut self, button: MouseButton) {
        let state = self.map.control_state();

        if displacement(state.mouse_state.button_pressed_position(button), state.mouse_state.cursor_position) <= self.settings.max_click_displacement {
            self.trigger(ClickEvent {cursor_position: self.map.control_state().mouse_state.cursor_position, button});
        }

        self.map.control_state_mut().mouse_state.capture_button_released(button);
    }

    fn trigger<E>(&mut self, event: E)
        where E: Copy,
              HandlerStore: TypedHandlerStore<E>
    {
        let store = if let Some(s) = self.map.handler_store().upgrade() { s } else { return; };
        TypedHandlerStore::trigger_event(&store, event, &mut self.map);
    }

    fn cursor_moved(&mut self, x: i32, y: i32) {
        let state = &self.map.control_state().mouse_state;
        if state.any_button_pressed() {
            let prev_position = state.cursor_position;
            let dx = x - prev_position[0];
            let dy = -(y - prev_position[1]);
            let curr_cursor_position = [x, y];

            // we do have a way to process drag event with multiple mouse buttons pressed, so
            // we precess them in priority list. If left button pressed, we ignore others etc.
            // I cannot think of a use case when firing drag event for different buttons at the
            // same time can be useful...
            // If this logic for some use cases will not be enough, it's better to change the
            // event struct to provide information about all the buttons, rather then firing
            // several events at once.
            if state.button_pressed(MouseButton::Left) {
                self.trigger(DragEvent {dx, dy, button: MouseButton::Left, curr_cursor_position});
            } else if state.button_pressed(MouseButton::Right) {
                self.trigger(DragEvent {dx, dy, button: MouseButton::Right, curr_cursor_position});
            } else if state.button_pressed(MouseButton::Middle) {
                self.trigger(DragEvent {dx, dy, button: MouseButton::Middle, curr_cursor_position});
            }
        }

        self.map.control_state_mut().mouse_state.cursor_position = [x, y];
    }

    fn wheel(&mut self, delta: MouseScrollDelta) {
        if self.map.control_state().last_zoom_time.elapsed().as_millis() < self.settings.zoom_delay as u128 {
            return;
        }

        let dy = match delta {
            MouseScrollDelta::LineDelta(_, dy) => dy,
            MouseScrollDelta::PixelDelta(v) => v.y as f32,
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
        self.trigger(ZoomEvent {delta, cursor_position: self.map.control_state().mouse_state.cursor_position});

        self.map.control_state_mut().last_zoom_time = instant::Instant::now();
    }
}

fn displacement(p1: [i32; 2], p2: [i32; 2]) -> i32 {
    (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
}
