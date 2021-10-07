use crate::Point;
use crate::map::Map;
use std::rc::{Weak, Rc};
use std::cell::RefCell;

pub enum MapEvent {
    Click(ClickEvent),
    DoubleClick(DoubleClickEvent),
    Drag {dx: i32, dy: i32},
    RightButtonDrag {dx: i32, dy: i32, cursor_position: [i32; 2]},
    MiddleButtonDrag {dx: i32, dy: i32},
    Zoom {delta: f32, cursor_position: [i32; 2]},
}

pub trait Event {}

#[derive(Debug, Clone, Copy)]
pub struct ClickEvent {
    pub cursor_position: [i32; 2],
}

#[derive(Debug, Clone, Copy)]
pub struct DoubleClickEvent {
}

impl Event for ClickEvent {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventState {
    Continue,
    Final,
}

#[derive(Default)]
pub struct HandlerStore {
    pub left_click: Vec<Box<dyn Fn(ClickEvent, &Map) -> EventState>>,
    pub double_click: Vec<Box<dyn Fn(DoubleClickEvent, &Map) -> EventState>>,
}

pub trait TypedHandlerStore<E> {
    fn get_store(&self) -> &Vec<Box<dyn Fn(E, &Map) -> EventState>>;
    fn get_store_mut(&mut self) -> &mut Vec<Box<dyn Fn(E, &Map) -> EventState>>;
}

impl TypedHandlerStore<ClickEvent> for HandlerStore {
    fn get_store(&self) -> &Vec<Box<dyn Fn(ClickEvent, &Map) -> EventState>> {
        &self.left_click
    }

    fn get_store_mut(&mut self) -> &mut Vec<Box<dyn Fn(ClickEvent, &Map) -> EventState>> {
        &mut self.left_click
    }
}

impl TypedHandlerStore<DoubleClickEvent> for HandlerStore {
    fn get_store(&self) -> &Vec<Box<dyn Fn(DoubleClickEvent, &Map) -> EventState>> {
        &self.double_click
    }

    fn get_store_mut(&mut self) -> &mut Vec<Box<dyn Fn(DoubleClickEvent, &Map) -> EventState>> {
        &mut self.double_click
    }
}

pub trait EventListener<E>
    where E: Copy,
          HandlerStore: TypedHandlerStore<E>
{
    fn handler_store(&self) -> &HandlerStore;
    fn handler_store_mut(&mut self) -> &mut HandlerStore;

    fn on(& mut self, handler: Box<dyn Fn(E, &Map) -> EventState>) {
        TypedHandlerStore::<E>::get_store_mut(self.handler_store_mut()).push(handler);
    }

    fn trigger_event(&self, event: E, map: &Map) {
        let store = TypedHandlerStore::<E>::get_store(self.handler_store());

        for i in 0..store.len() {
            let handler = &store[i];
            let state = handler.clone()(event, map);
            if state == EventState::Final {
                break;
            }
        }
    }
}
