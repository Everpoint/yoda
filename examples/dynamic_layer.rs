use yoda::event::{MapEvent, ClickEvent, EventState};
use yoda::symbol::CircleSymbol;
use yoda::runtime::native::NativeRuntime;
use yoda::layer::DynamicLayer;
use std::rc::Rc;
use yoda::map::Map;
use std::cell::RefCell;

fn main() {
    let mut runtime = NativeRuntime::new(&|b| b.with_title("Simple yoda map example"));

    let symbol = CircleSymbol { size: 20.0, color: [0.0, 0.7, 0.7, 1.0], program: None };
    let layer = Rc::new(RefCell::new(DynamicLayer::new(symbol)));

    let mut map = runtime.map_mut();
    map.add_layer(layer.clone());

    map.on_left_click(move |e, map| {
        let map_position = map.position().get_map_position(&e.cursor_position);
        layer.borrow_mut().add([map_position[0], map_position[1], 0.0]);

        EventState::Final
    });

    runtime.run();
}
