use yoda::event::{ClickEvent, EventState, EventListener};
use yoda::symbol::CircleSymbol;
use yoda::runtime::native::NativeRuntime;
use yoda::layer::DynamicLayer;
use std::rc::Rc;
use std::cell::RefCell;
use winit::event::MouseButton;

fn main() {
    let mut runtime = NativeRuntime::new(&|b| b.with_title("Simple yoda map example"));

    let symbol = CircleSymbol { size: 10.0, color: [0.0, 0.7, 0.7, 1.0], program: None };
    let layer = Rc::new(RefCell::new(DynamicLayer::new(symbol)));

    let mut map = runtime.map_mut();
    map.add_layer(layer.clone());

    let mut counter = Rc::new(RefCell::new(0));

    let layer_copy = layer.clone();
    map.on(Rc::new(move |e: ClickEvent, map| {
        if e.button != MouseButton::Left {
            return EventState::Continue;
        }

        let map_position = map.position().get_map_position(&e.cursor_position);
        layer_copy.borrow_mut().add([map_position[0], map_position[1], 0.0]);

        // returning EventState::Continue allows the next handler to be called
        EventState::Continue
    }));

    let layer_copy = layer.clone();
    let handler_id = Rc::new(RefCell::new(0));
    let handler_id_copy = handler_id.clone();

    *handler_id.borrow_mut() = map.on(Rc::new(move |e: ClickEvent, map| {
        let map_position = map.position().get_map_position(&e.cursor_position);
        layer_copy.borrow_mut().add([map_position[0] + 100., map_position[1], 0.0]);

        *counter.borrow_mut() += 1;

        if *counter.borrow() == 5 {
            EventListener::<ClickEvent>::off(map, *handler_id_copy.borrow());
        }

        // no more handlers will be called after this
        EventState::Final
    }));

    // this handler will be called only after the previous handler is removed (after counter counts
    // to 5)
    map.on(Rc::new(move |e: ClickEvent, map| {
        let map_position = map.position().get_map_position(&e.cursor_position);
        layer.borrow_mut().add([map_position[0] - 100., map_position[1], 0.0]);

        EventState::Final
    }));

    runtime.run();
}
