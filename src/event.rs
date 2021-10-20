use crate::map::Map;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use winit::event::MouseButton;

#[derive(Debug, Clone, Copy)]
pub struct ClickEvent {
    pub cursor_position: [i32; 2],
    pub button: MouseButton,
}

#[derive(Debug, Clone, Copy)]
pub struct DoubleClickEvent {}

#[derive(Debug, Clone, Copy)]
pub struct DragEvent {
    pub dx: i32,
    pub dy: i32,
    pub button: MouseButton,
    pub curr_cursor_position: [i32; 2],
}

#[derive(Debug, Clone, Copy)]
pub struct ZoomEvent {
    pub delta: f32,
    pub cursor_position: [i32; 2],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventState {
    Continue,
    Final,
}

#[derive(Default)]
pub struct HandlerStore {
    next_id: usize,
    pub click: ClickEventStore,
    pub double_click: DoubleClickEventStore,
    pub drag: DragEventStore,
    pub zoom: ZoomEventStore,
}

impl HandlerStore {
    fn next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }
}

type EventStore<E> = Vec<(usize, Rc<dyn Fn(E, &mut Map) -> EventState>)>;
pub trait TypedHandlerStore<E: Copy> {
    fn get_store(&self) -> &EventStore<E>;
    fn get_store_mut(&mut self) -> &mut EventStore<E>;

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

type ClickEventStore = Vec<(usize, Rc<dyn Fn(ClickEvent, &mut Map) -> EventState>)>;
impl TypedHandlerStore<ClickEvent> for HandlerStore {
    fn get_store(&self) -> &ClickEventStore {
        &self.click
    }

    fn get_store_mut(&mut self) -> &mut ClickEventStore {
        &mut self.click
    }
}

type DoubleClickEventStore = Vec<(usize, Rc<dyn Fn(DoubleClickEvent, &mut Map) -> EventState>)>;
impl TypedHandlerStore<DoubleClickEvent> for HandlerStore {
    fn get_store(&self) -> &DoubleClickEventStore {
        &self.double_click
    }

    fn get_store_mut(&mut self) -> &mut DoubleClickEventStore {
        &mut self.double_click
    }
}

type DragEventStore = Vec<(usize, Rc<dyn Fn(DragEvent, &mut Map) -> EventState>)>;
impl TypedHandlerStore<DragEvent> for HandlerStore {
    fn get_store(&self) -> &DragEventStore {
        &self.drag
    }

    fn get_store_mut(&mut self) -> &mut DragEventStore {
        &mut self.drag
    }
}

type ZoomEventStore = Vec<(usize, Rc<dyn Fn(ZoomEvent, &mut Map) -> EventState>)>;
impl TypedHandlerStore<ZoomEvent> for HandlerStore {
    fn get_store(&self) -> &ZoomEventStore {
        &self.zoom
    }

    fn get_store_mut(&mut self) -> &mut ZoomEventStore {
        &mut self.zoom
    }
}

pub trait EventListener<E>
where
    E: Copy,
    HandlerStore: TypedHandlerStore<E>,
{
    fn handler_store(&self) -> Weak<RefCell<HandlerStore>>;

    fn on(&self, handler: Rc<dyn Fn(E, &mut Map) -> EventState>) -> usize {
        let store = self.handler_store().upgrade().unwrap();
        let id = store.borrow_mut().next_id();
        TypedHandlerStore::<E>::get_store_mut(&mut *store.borrow_mut()).push((id, handler));
        id
    }

    fn off(&self, handler_id: usize) {
        let store = self.handler_store().upgrade().unwrap();
        let position = TypedHandlerStore::<E>::get_store_mut(&mut *store.borrow_mut())
            .iter()
            .position(|(id, _)| *id == handler_id);
        if let Some(index) = position {
            TypedHandlerStore::<E>::get_store_mut(&mut *store.borrow_mut()).remove(index);
        }
    }
}
