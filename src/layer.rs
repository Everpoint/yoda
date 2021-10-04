use crate::render_target::RenderTarget;
use crate::{Point, Point3};
use crate::symbol::{CircleSymbol, CirclePointVertex, Symbol};
use glium::{Surface, DrawParameters, Blend};
use crate::map::MapPosition;
use glium::index::PrimitiveType;

pub trait Layer {
    fn draw(&mut self, target: &mut RenderTarget, position: &MapPosition);
}

pub struct StaticLayer<G, S: Symbol<G>> {
    features: Vec<G>,
    symbol: S,

    vertex_buffer: Option<glium::VertexBuffer<S::Vertex>>,
    index_buffer: Option<glium::IndexBuffer<u32>>,
}

impl<G, S: Symbol<G>> StaticLayer<G, S> {
    pub fn new(symbol: S, features: Vec<G>) -> Self {
        Self {features, vertex_buffer: None, index_buffer: None, symbol}
    }
}

impl<G, S: Symbol<G>> StaticLayer<G, S> {
    fn prepare_buffer(&mut self, target: &mut RenderTarget) {
        if self.vertex_buffer.is_none() {
            let mut vertices = vec![];
            let mut indices = vec![];
            for p in &self.features {
                let (mut geom_vertices, mut geom_indexes) = self.symbol.convert(&p);
                vertices.append(&mut geom_vertices);
                if let Some(mut i) = geom_indexes {
                    indices.append(&mut i);
                }
            }

            self.vertex_buffer = Some(glium::VertexBuffer::new(target.display(), &vertices).unwrap());
            if indices.len() > 0 {
                self.index_buffer = Some(glium::IndexBuffer::new(target.display(), PrimitiveType::TrianglesList, &indices).unwrap());
            }
        }
    }
}

impl<G, S: Symbol<G>> Layer for StaticLayer<G, S> {
    fn draw(&mut self, target: &mut RenderTarget, position: &MapPosition) {
        self.symbol.compile(target.display());
        self.prepare_buffer(target);

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

        let mut draw_parameters = DrawParameters::default();
        draw_parameters.blend = Blend::alpha_blending();
        if let Some(buffer) = &self.index_buffer {
            target.frame().draw(self.vertex_buffer.as_ref().unwrap(), buffer, self.symbol.program(), &trans, &draw_parameters).unwrap();
        } else {
            let indices = &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
            target.frame().draw(self.vertex_buffer.as_ref().unwrap(), indices, self.symbol.program(), &trans, &draw_parameters).unwrap();
        };
    }
}
