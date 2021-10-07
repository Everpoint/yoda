mod dynamic;
pub use dynamic::DynamicLayer;

use crate::render_target::RenderTarget;
use crate::{Point, Point3};
use crate::symbol::{CircleSymbol, CirclePointVertex, Symbol};
use crate::map::MapPosition;
use glow::{HasContext, Buffer, VertexArray, Context};
use crate::gl::GlBuffer;

pub trait Layer {
    fn draw(&mut self, target: &RenderTarget, position: &MapPosition);
}

pub struct StaticLayer<G, S: Symbol<G>> {
    features: Vec<G>,
    symbol: S,
    buffer: Option<GlBuffer>,
}

impl<G, S: Symbol<G>> StaticLayer<G, S> {
    pub fn new(symbol: S, features: Vec<G>) -> Self {
        Self {features, symbol, buffer: None}
    }

    pub fn clean(&mut self, gl: &Context) {
        // buffers must be deleted
        todo!()
    }
}

impl<G, S: Symbol<G>> StaticLayer<G, S> {
    fn prepare_buffer(&mut self, target: &RenderTarget) {
        if self.buffer.is_none() {
            let mut vertices = vec![];
            let mut indices = vec![];
            for p in &self.features {
                let (mut geom_vertices, mut geom_indexes) = self.symbol.convert(&p);
                vertices.append(&mut geom_vertices);
                if let Some(mut i) = geom_indexes {
                    indices.append(&mut i);
                }
            }

            let indices = if indices.len() == 0 { None } else { Some(&indices[..]) };
            let gl = target.context();
            self.buffer = Some(GlBuffer::create(gl, &vertices, indices));
        }
    }
}

impl<G, S: Symbol<G>> Layer for StaticLayer<G, S> {
    fn draw(&mut self, target: &RenderTarget, position: &MapPosition) {
        self.symbol.compile(target.context());
        self.prepare_buffer(target);
        let t = position.matrix();
        let (width, height) = target.get_dimensions();
        let t = [
                    t[(0, 0)], t[(0, 1)], t[(0, 2)], t[(0, 3)],
                    t[(1, 0)], t[(1, 1)], t[(1, 2)], t[(1, 3)],
                    t[(2, 0)], t[(2, 1)], t[(2, 2)], t[(2, 3)],
                    t[(3, 0)], t[(3, 1)], t[(3, 2)], t[(3, 3)],
                ];

        unsafe {
            let gl = target.context();
            gl.use_program(Some(*self.symbol.program().unwrap()));

            let transformation_location = gl.get_uniform_location(*self.symbol.program().unwrap(), "transformation").unwrap();
            gl.uniform_matrix_4_f32_slice(Some(&transformation_location), false, &t);

            if let Some(screen_size_location) = gl.get_uniform_location(*self.symbol.program().unwrap(), "screen_size") {
                gl.uniform_2_f32(Some(&screen_size_location), width as f32, height as f32);
            }

            let buffer = self.buffer.as_ref().unwrap();
            gl.bind_vertex_array(Some(buffer.vertex_array));
            if let Some(_) = buffer.index_buffer {
                gl.draw_elements(glow::TRIANGLES, buffer.vertex_count as i32, glow::UNSIGNED_INT, 0);
            } else {
                gl.draw_arrays(glow::TRIANGLES, 0, buffer.vertex_count as i32);
            }

            gl.bind_vertex_array(None);
        }
    }
}

unsafe fn to_bytes<T>(p: &T, size: usize) -> &[u8] {
    std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        size,
    )
}
