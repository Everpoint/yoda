use crate::render_target::RenderTarget;
use crate::{Point, Point3};
use crate::symbol::{CircleSymbol, CirclePointVertex};
use glium::Surface;
use crate::map::MapPosition;

pub trait Layer {
    fn draw(&mut self, target: &mut RenderTarget, position: &MapPosition);
}

pub struct StaticLayer {
    points: Vec<Point3>,
    buffer: Option<glium::VertexBuffer<CirclePointVertex>>,
    symbol: CircleSymbol,
}

impl StaticLayer {
    pub fn new(points: Vec<Point3>) -> Self {
        let mut symbol = CircleSymbol { size: 3.0, color: [0.0, 0.7, 0.7, 1.0], program: None };
        Self {points, buffer: None, symbol}
    }
}

impl StaticLayer {
    fn prepare_buffer(&mut self, target: &mut RenderTarget) {
        if self.buffer.is_none() {
            let vertexes: Vec<CirclePointVertex> = self.points.iter().map(|p| self.symbol.convert(p)).flatten().collect();
            self.buffer = Some(glium::VertexBuffer::new(target.display(), &vertexes).unwrap());
        }
    }
}

impl Layer for StaticLayer {
    fn draw(&mut self, target: &mut RenderTarget, position: &MapPosition) {
        self.symbol.compile(target.display());
        self.prepare_buffer(target);
        let indexes = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let t = position.matrix();
        let screen_size = target.frame().get_dimensions();

        let trans = uniform! {
            transformation: [
                [t[(0, 0)], t[(0, 1)], t[(0, 2)], t[(0, 3)]],
                [t[(1, 0)], t[(1, 1)], t[(1, 2)], t[(1, 3)]],
                [t[(2, 0)], t[(2, 1)], t[(2, 2)], t[(2, 3)]],
                [t[(3, 0)], t[(3, 1)], t[(3, 2)], t[(3, 3)]],
            ],
            screen_size: [screen_size.0 as f32, screen_size.1 as f32],
        };

        target.frame().draw(self.buffer.as_ref().unwrap(), &indexes, self.symbol.program(), &trans, &Default::default()).unwrap();
    }
}
