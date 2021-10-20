use crate::map::Map;
use std::rc::Rc;
use crate::event::{EventListener, DragEvent, EventState, ZoomEvent};
use std::cell::RefCell;
use winit::event::MouseButton;

pub struct DefaultMapControl {
    map: Option<Rc<RefCell<Map>>>,
    handlers: HandlerIds,
}

#[derive(Debug, Default)]
struct HandlerIds {
    drag: usize,
    zoom: usize,
}

impl DefaultMapControl {
    pub fn new() -> Self {
        Self {map: None, handlers: HandlerIds::default()}
    }

    pub fn is_attached(&self) -> bool {
        self.map.is_some()
    }

    pub fn attach(&mut self, map_cell: Rc<RefCell<Map>>) {
        let map = map_cell.borrow();
        self.handlers.drag = map.on(Rc::new(handle_drag));
        self.handlers.zoom = map.on(Rc::new(handle_zoom));

        drop(map);
        self.map = Some(map_cell);
    }

    pub fn detach(&mut self) {
        if let Some(map) = &self.map {
            EventListener::<DragEvent>::off(&*map.borrow(), self.handlers.drag);
            EventListener::<ZoomEvent>::off(&*map.borrow(), self.handlers.zoom);
            self.handlers = HandlerIds::default();
            self.map = None;
        }
    }
}

impl Drop for DefaultMapControl {
    fn drop(&mut self) {
        self.detach();
    }
}

fn handle_zoom(e: ZoomEvent, map: &mut Map) -> EventState {
    map.position_mut().zoom(e.delta, e.cursor_position);
    EventState::Final
}

fn handle_drag(e: DragEvent, map: &mut Map) -> EventState {
    match e.button {
        MouseButton::Left => handle_left_button_drag(e.dx, e.dy, map),
        MouseButton::Right => handle_right_button_drag(e.dx, e.dy, map, e.curr_cursor_position),
        MouseButton::Middle => handle_middle_button_drag(e.dx, e.dy, map),
        MouseButton::Other(_) => EventState::Continue,
    }
}

fn handle_left_button_drag(dx: i32, dy: i32, map: &mut Map) -> EventState {
    map.position_mut().translate_px(dx, dy);
    EventState::Final
}

fn handle_middle_button_drag(_: i32, dy: i32, map: &mut Map) -> EventState {
    const ANGLE_STEP: f32 = 0.005;
    map.position_mut().rotate(dy as f32 * ANGLE_STEP, 0.0);

    EventState::Final
}

fn handle_right_button_drag(dx: i32, dy: i32, map: &mut Map, cursor_position: [i32; 2]) -> EventState {
    let position = map.position_mut();
    let center = position.center();
    let position_on_map = position.get_map_position(&cursor_position);
    let v1 = [center[0] - position_on_map[0], center[1] - position_on_map[1]];
    let dx = dx as f32 * position.resolution();
    let dy = dy as f32 * position.resolution();

    let v2 = [v1[0] + dx, v1[1] + dy];
    let v1_len = (v1[0] * v1[0] + v1[1] * v1[1]).sqrt();
    let v2_len = (v2[0] * v2[0] + v2[1] * v2[1]).sqrt();
    if v1_len == 0. || v2_len == 0. {
        return EventState::Final;
    }

    let prod = (v1[0] * v2[0] + v1[1] * v2[1]) / v1_len / v2_len;
    if prod > 1. {
        return EventState::Final;
    }

    let mut angle = prod.acos();
    if v1[0] * v2[1] - v1[1] * v2[0] > 0.0 {
        angle = -angle;
    }

    position.rotate(0.0, angle);

    EventState::Final
}
