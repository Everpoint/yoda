use crate::render_target::RenderTarget;
use crate::{Point, Point3};
use crate::symbol::{CircleSymbol, CirclePointVertex, Symbol};
use crate::map::MapPosition;
use glow::{HasContext, Buffer, VertexArray};

pub trait Layer {
    fn draw(&mut self, target: &mut RenderTarget, position: &MapPosition);
}

pub struct StaticLayer<G, S: Symbol<G>> {
    features: Vec<G>,
    symbol: S,

    vertex_buffer: Option<Buffer>,
    index_buffer: Option<Buffer>,
    vertex_array: Option<VertexArray>,
    vertex_count: u32,
}

impl<G, S: Symbol<G>> StaticLayer<G, S> {
    pub fn new(symbol: S, features: Vec<G>) -> Self {
        Self {features, vertex_buffer: None, index_buffer: None, vertex_array: None, symbol, vertex_count: 0}
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

            let gl = target.context();

            unsafe {

                let error = gl.get_error();
                if error != glow::NO_ERROR {
                    panic!("ERROR: {}", error);
                }
                let vertex_array = gl.create_vertex_array().unwrap();
                gl.bind_vertex_array(Some(vertex_array));

                let vertex_size = std::mem::size_of::<S::Vertex>();
                eprintln!("{}", vertex_size);
                let vertex_buffer = gl.create_buffer().unwrap();
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, to_bytes(&vertices[0], vertex_size * vertices.len()), glow::STATIC_DRAW);

                gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, vertex_size as i32, 0);
                gl.enable_vertex_attrib_array(0);

                gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, vertex_size as i32, 3 * 4);
                gl.enable_vertex_attrib_array(1);

                gl.vertex_attrib_pointer_f32(2, 4, glow::FLOAT, false, vertex_size as i32, (3 + 2) * 4);
                gl.enable_vertex_attrib_array(2);

                gl.vertex_attrib_pointer_f32(3, 1, glow::FLOAT, false, vertex_size as i32, (3 + 2 + 4) * 4);
                gl.enable_vertex_attrib_array(3);

                self.vertex_buffer = Some(vertex_buffer);
                self.vertex_count = vertices.len() as u32;

                if indices.len() > 0 {
                    let index_buffer = gl.create_buffer().unwrap();
                    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer));
                    gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, &to_bytes(&indices[0], 4 * indices.len()), glow::STATIC_DRAW);

                    self.index_buffer = Some(index_buffer);
                }

                gl.bind_buffer(glow::ARRAY_BUFFER, None);
                gl.bind_vertex_array(None);

                let error = gl.get_error();
                if error != glow::NO_ERROR {
                    panic!("ERROR: {}", error);
                }

                self.vertex_array = Some(vertex_array);
            }
        }
    }
}

unsafe fn to_bytes<T>(p: &T, size: usize) -> &[u8] {
    std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        size,
    )
}

impl<G, S: Symbol<G>> Layer for StaticLayer<G, S> {
    fn draw(&mut self, target: &mut RenderTarget, position: &MapPosition) {
        let gl = target.context();

        unsafe {
            let error = gl.get_error();
            if error != glow::NO_ERROR {
                panic!("ERROR: {}", error);
            }
        }

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
            gl.use_program(Some(*self.symbol.program()));

            let error = gl.get_error();
            if error != glow::NO_ERROR {
                panic!("ERROR: {}", error);
            }

            let transformation_location = gl.get_uniform_location(*self.symbol.program(), "transformation").unwrap();
            gl.uniform_matrix_4_f32_slice(Some(&transformation_location), false, &t);

            let screen_size_location = gl.get_uniform_location(*self.symbol.program(), "screen_size").unwrap();
            gl.uniform_2_f32(Some(&screen_size_location), width as f32, height as f32);


            let error = gl.get_error();
            if error != glow::NO_ERROR {
                panic!("ERROR: {}", error);
            }
            gl.bind_vertex_array(self.vertex_array);
            if let Some(_) = self.index_buffer {
                gl.draw_elements(glow::TRIANGLES, self.vertex_count as i32, glow::UNSIGNED_INT, 0);
            } else {
                gl.draw_arrays(glow::TRIANGLES, 0, self.vertex_count as i32);
            }

            gl.bind_vertex_array(None);

            let error = gl.get_error();
            if error != glow::NO_ERROR {
                panic!("ERROR: {}", error);
            }
        }
    }
}
