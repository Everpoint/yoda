use crate::Point;
use crate::map::Map;
use std::rc::{Weak, Rc};
use std::cell::RefCell;

pub enum MapEvent {
    Click(ClickEvent),
    Drag {dx: i32, dy: i32},
    RightButtonDrag {dx: i32, dy: i32, cursor_position: [i32; 2]},
    MiddleButtonDrag {dx: i32, dy: i32},
    Zoom {delta: f32, cursor_position: [i32; 2]},
}

#[derive(Debug, Clone, Copy)]
pub struct ClickEvent {
    pub cursor_position: [i32; 2],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventState {
    Continue,
    Final,
}

#[derive(Default)]
pub struct HandlerStore {
    pub left_click: Vec<Rc<dyn Fn(ClickEvent, &mut Map) -> EventState>>,
}
