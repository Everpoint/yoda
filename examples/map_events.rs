use yoda::event::{ClickEvent, EventState, EventListener};
use yoda::symbol::CircleSymbol;
use yoda::runtime::native::NativeRuntime;
use std::rc::Rc;
use std::cell::RefCell;
use winit::event::MouseButton;
use yoda::layer::{StaticLayer, Layer};
use yoda::render_target::RenderTarget;

fn main() {
    let mut runtime = NativeRuntime::new(&|b| b.with_title("Simple yoda map example"));

    let symbol = CircleSymbol { size: 10.0, color: [0.0, 0.7, 0.7, 1.0], program: None };
    let layer = Rc::new(RefCell::new(StaticLayer::new(symbol, vec![])));

    let context = runtime.context();
    let map = runtime.map_mut();
    map.add_layer(layer.clone());

    let layer_copy = layer;
    map.on(Rc::new(move |e: ClickEvent, map| {
        if e.button != MouseButton::Left {
            return EventState::Continue;
        }

        let cursor_position = e.cursor_position;
        let target = RenderTarget::new(context.clone(), (map.position().width_px() as u32, map.position().height_px() as u32));
        let mut layer = layer_copy.borrow_mut();
        if let Some(feature) = layer.feature_at_point(&target, cursor_position, map.position()) {
            layer.remove(feature);
            return EventState::Final;
        }


        let map_position = map.position().get_map_position(&cursor_position);
        layer.add([map_position[0], map_position[1], 0.0]);

        // returning EventState::Continue allows the next handler to be called
        EventState::Continue
    }));

    runtime.run();
}
