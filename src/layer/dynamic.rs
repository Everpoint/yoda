use crate::symbol::Symbol;
use crate::layer::Layer;
use crate::render_target::RenderTarget;
use crate::map::MapPosition;
use crate::gl::GlBuffer;
use glow::HasContext;

pub struct DynamicLayer<G, S: Symbol<G>> {
    symbol: S,
    features: Vec<G>,
}

impl<G, S: Symbol<G>> DynamicLayer<G, S> {
    pub fn new(symbol: S) -> Self {
        Self {symbol, features: vec![]}
    }

    fn prepare_buffer(&mut self, target: &RenderTarget) -> GlBuffer {
        let mut vertices = vec![];
        let mut indices = vec![];
        for p in &self.features {
            let (mut geom_vertices, geom_indexes) = self.symbol.convert(&p);
            vertices.append(&mut geom_vertices);
            if let Some(mut i) = geom_indexes {
                indices.append(&mut i);
            }
        }

        let indices = if indices.len() == 0 { None } else { Some(&indices[..]) };
        let gl = target.context();
        GlBuffer::create(gl, &vertices, indices)
    }

    pub fn add(&mut self, feature: G) {
        self.features.push(feature);
    }
}

impl<G, S: Symbol<G>> Layer for DynamicLayer<G, S> {
    fn draw(&mut self, target: &RenderTarget, position: &MapPosition) {
        if self.features.len() == 0 {
            return;
        }

        let gl = target.context();
        self.symbol.compile(gl);

        let buffer = self.prepare_buffer(target);
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