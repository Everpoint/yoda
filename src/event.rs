use crate::Point;

pub enum MapEvent {
    Drag {dx: i32, dy: i32},
    RightButtonDrag {dx: i32, dy: i32, cursor_position: [i32; 2]},
    MiddleButtonDrag {dx: i32, dy: i32},
    Zoom {delta: f32, cursor_position: [i32; 2]},
}
