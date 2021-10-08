use crate::map::Map;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use std::collections::HashMap;

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
    next_id: usize,
    pub left_click: Vec<(usize, Rc<dyn Fn(ClickEvent, &mut Map) -> EventState>)>,
    pub double_click: Vec<(usize, Rc<dyn Fn(DoubleClickEvent, &mut Map) -> EventState>)>,
}

impl HandlerStore {
    fn next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }
}

pub trait TypedHandlerStore<E: Copy> {
    fn get_store(&self) -> &Vec<(usize, Rc<dyn Fn(E, &mut Map) -> EventState>)>;
    fn get_store_mut(&mut self) -> &mut Vec<(usize, Rc<dyn Fn(E, &mut Map) -> EventState>)>;

    fn trigger_event(store: &Rc<RefCell<Self>>, event: E, map: &mut Map) {
        let mut handlers = vec![];
        for (_, handler) in store.borrow().get_store() {
            handlers.push(handler.clone());
        }

        for handler in handlers {
            let state = handler(event, map);
            if state == EventState::Final {
                break;
            }
        }
    }
}

impl TypedHandlerStore<ClickEvent> for HandlerStore {
    fn get_store(&self) -> &Vec<(usize, Rc<dyn Fn(ClickEvent, &mut Map) -> EventState>)> {
        &self.left_click
    }

    fn get_store_mut(&mut self) -> &mut Vec<(usize, Rc<dyn Fn(ClickEvent, &mut Map) -> EventState>)> {
        &mut self.left_click
    }
}

impl TypedHandlerStore<DoubleClickEvent> for HandlerStore {
    fn get_store(&self) -> &Vec<(usize, Rc<dyn Fn(DoubleClickEvent, &mut Map) -> EventState>)> {
        &self.double_click
    }

    fn get_store_mut(&mut self) -> &mut Vec<(usize, Rc<dyn Fn(DoubleClickEvent, &mut Map) -> EventState>)> {
        &mut self.double_click
    }
}

pub trait EventListener<E>
    where E: Copy,
          HandlerStore: TypedHandlerStore<E>
{
    fn handler_store(&self) -> Weak<RefCell<HandlerStore>>;

    fn on(&mut self, handler: Rc<dyn Fn(E, &mut Map) -> EventState>) -> usize {
        let store = self.handler_store().upgrade().unwrap();
        let id = store.borrow_mut().next_id();
        TypedHandlerStore::<E>::get_store_mut(&mut *store.borrow_mut()).push((id, handler));
        id
    }

    fn off(&mut self, handler_id: usize) {
        let store = self.handler_store().upgrade().unwrap();
        let position = TypedHandlerStore::<E>::get_store_mut(&mut *store.borrow_mut()).iter().position(|(id, _)| *id == handler_id);
        if let Some(index) = position {
            TypedHandlerStore::<E>::get_store_mut(&mut *store.borrow_mut()).remove(index);
        }
    }
}
