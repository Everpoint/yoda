use yoda::layer::StaticLayer;

use yoda::symbol::{CircleSymbol, LineSymbol, PolygonSymbol};

use std::cell::RefCell;
use std::rc::Rc;
use yoda::runtime::native::NativeRuntime;

fn main() {
    let line_symbol = LineSymbol {
        width: 3.0,
        color: [0.5, 0.2, 0.0, 1.0],
        program: None,
    };
    let line = vec![
        [0.0, 0.0, 0.0],
        [100.0, 100.0, 0.0],
        [100.0, 0.0, 0.0],
        [200.0, 0.0, 0.0],
        [200.0, 100.0, 0.0],
    ];
    let line_layer = StaticLayer::new(line_symbol, vec![line]);

    let point_symbol = CircleSymbol {
        size: 20.0,
        color: [0.0, 0.7, 0.7, 1.0],
        program: None,
    };
    let points = vec![
        [0.0, 0.0, 0.0],
        [100.0, 100.0, 0.0],
        [100.0, 0.0, 0.0],
        [0.0, 100.0, 0.0],
    ];
    let point_layer = StaticLayer::new(point_symbol, points);

    let polygon_symbol = PolygonSymbol {
        fill_color: [0.0, 0.5, 0.3, 0.5],
        stroke_color: [0.0, 0.5, 0.5, 1.0],
        stroke_width: 2.5,
        program: None,
    };

    let polygon = vec![
        vec![
            [-150.0, -150.0, 0.0],
            [-150.0, 150.0, 0.0],
            [150.0, 150.0, 0.0],
            [150.0, -150.0, 0.0],
        ],
        vec![[-30.0, -30.0, 0.0], [30.0, -30.0, 0.0], [0.0, 30.0, 0.0]],
    ];
    let polygon_layer = StaticLayer::new(polygon_symbol, vec![polygon]);

    let mut runtime = NativeRuntime::new(&|b| b.with_title("Simple yoda map example"));

    let map = runtime.map_mut();
    map.add_layer(Rc::new(RefCell::new(line_layer)));
    map.add_layer(Rc::new(RefCell::new(point_layer)));
    map.add_layer(Rc::new(RefCell::new(polygon_layer)));

    runtime.run();
}
