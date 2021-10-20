
use yoda::layer::StaticLayer;

use yoda::symbol::{CircleSymbol};




use yoda::runtime::native::NativeRuntime;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shape_points = shapefile::reader::ShapeReader::from_path("./examples/data/points_wm.shp").unwrap().read().unwrap();
    let mut points = vec![];
    let mut bbox: [f32; 4] = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
    for shape in shape_points {
        match shape {
            shapefile::Shape::Point(p) => {
                points.push([p.x as f32, p.y as f32, 0.0]);
                bbox[0] = bbox[0].min(p.x as f32);
                bbox[1] = bbox[1].min(p.y as f32);
                bbox[2] = bbox[2].max(p.x as f32);
                bbox[3] = bbox[3].max(p.y as f32);
            },
            _ => {},
        }
    }

    let width = bbox[2] - bbox[0];
    let height = bbox[3] - bbox[1];
    let len = points.len();
    for i in 0..9 {
        for j in 0..9 {
            if i == 0 && j == 0 { continue; }

            for z in 0..len {
                let p = points[z];
                points.push([p[0] + i as f32 * width, p[1] + j as f32 * height, 0.0]);
                // points.push([p[0], p[1], j as f32 * 30000.0]);
            }
        }
    }

    eprintln!("Loaded {} points. ", points.len());

    let symbol = CircleSymbol { size: 5.0, color: [0.0, 0.7, 0.7, 1.0], program: None };
    let layer = StaticLayer::new(symbol, points);

    let mut runtime = NativeRuntime::new(&|b| b.with_title("Shapefile rendering example"));
    let map = runtime.map_mut();

    map.set_center((bbox[2] + bbox[0]) / 2.0, (bbox[3] + bbox[1]) / 2.0);
    map.set_resolution((bbox[2] - bbox[0]) / 800.0);

    map.add_layer(Rc::new(RefCell::new(layer)));

    runtime.run();
}