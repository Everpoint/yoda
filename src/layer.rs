use crate::render_target::RenderTarget;
use crate::Point;
use crate::symbol::{CircleSymbol, CirclePointVertex};
use glium::Surface;
use crate::map::MapPosition;

pub trait Layer {
    fn draw(&self, target: &mut RenderTarget, position: &MapPosition);
}

pub struct StaticLayer {
    points: Vec<Point>,
}

impl StaticLayer {
    pub fn new(points: Vec<Point>) -> Self {
        Self {points}
    }
}

impl Layer for StaticLayer {
    fn draw(&self, target: &mut RenderTarget, position: &MapPosition) {
        let mut symbol = CircleSymbol { size: 20.0, color: [1.0, 0.0, 0.0, 1.0], program: None };
        symbol.compile(target.display());

        let vertexes: Vec<CirclePointVertex> = self.points.iter().map(|p| symbol.convert(p)).flatten().collect();
        let buffer = glium::VertexBuffer::new(target.display(), &vertexes).unwrap();
        let indexes = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let t = position.matrix();

        let trans = uniform! {
            transformation: [
                [t[(0, 0)], t[(0, 1)], t[(0, 2)], t[(0, 3)]],
                [t[(1, 0)], t[(1, 1)], t[(1, 2)], t[(1, 3)]],
                [t[(2, 0)], t[(2, 1)], t[(2, 2)], t[(2, 3)]],
                [t[(3, 0)], t[(3, 1)], t[(3, 2)], t[(3, 3)]],
            ]
        };

        target.frame().draw(&buffer, &indexes, symbol.program(), &trans, &Default::default()).unwrap();
    }
}
