use crate::render_target::RenderTarget;
use crate::symbol::{Symbol};
use crate::map::MapPosition;
use glow::{HasContext, Context};
use crate::gl::GlBuffer;
use std::rc::Rc;

pub trait Layer {
    fn draw(&mut self, target: &RenderTarget, position: &MapPosition);
    fn feature_at_point(&self, _target: &RenderTarget, _screen_position: [i32; 2], _map_position: &MapPosition) -> Option<usize> {
        None
    }
}

pub struct StaticLayer<G, S: Symbol<G>> {
    features: Vec<G>,
    symbol: S,
    context: Option<Rc<Context>>,
    buffer: Option<GlBuffer>,
}

impl<G, S: Symbol<G>> StaticLayer<G, S> {
    pub fn new(symbol: S, features: Vec<G>) -> Self {
        Self {features, symbol, context: None, buffer: None}
    }

    pub fn clean(&mut self) {
        if let Some(gl) = &self.context {
            let buffer = self.buffer.as_ref().unwrap();
            unsafe {
                gl.delete_buffer(buffer.vertex_buffer);
                if let Some(index_buffer) = buffer.index_buffer {
                    gl.delete_buffer(index_buffer);
                }

                gl.delete_vertex_array(buffer.vertex_array);
            }

            self.buffer = None;
            self.context = None;
        }
    }

    fn set_context(&mut self, gl: Rc<Context>) {
        if let Some(context) = &self.context {
            if Rc::ptr_eq(context, &gl) {
                return;
            } else {
                self.clean();
            }
        }

        self.symbol.compile(&*gl);
        self.prepare_buffer(&*gl);

        self.context = Some(gl);
    }

    fn prepare_buffer(&mut self, gl: &Context) {
        if self.buffer.is_none() {
            let mut vertices = vec![];
            let mut indices = vec![];
            for (id, p) in self.features.iter().enumerate() {
                let (mut geom_vertices, geom_indexes) = self.symbol.convert(&p, id as u32);
                vertices.append(&mut geom_vertices);
                if let Some(mut i) = geom_indexes {
                    indices.append(&mut i);
                }
            }

            if vertices.len() == 0 {
                return;
            }

            let indices = if indices.len() == 0 { None } else { Some(&indices[..]) };
            self.buffer = Some(GlBuffer::create(&*gl, &vertices, indices));
        }
    }

    pub fn add(&mut self, feature: G) {
        self.clean();
        self.features.push(feature);
    }

    pub fn remove(&mut self, index: usize) {
        self.clean();
        self.features.remove(index);
    }

    pub fn draw_with_context(&self, gl: &Context, position: &MapPosition, width: u32, height: u32, drawing_mode: DrawingMode) {
        if self.buffer.is_none() {
            // No object produced any vertices, so skip drawing
            return;
        }

        let t = position.matrix();
        let t = [
            t[(0, 0)], t[(0, 1)], t[(0, 2)], t[(0, 3)],
            t[(1, 0)], t[(1, 1)], t[(1, 2)], t[(1, 3)],
            t[(2, 0)], t[(2, 1)], t[(2, 2)], t[(2, 3)],
            t[(3, 0)], t[(3, 1)], t[(3, 2)], t[(3, 3)],
        ];

        unsafe {
            gl.use_program(Some(*self.symbol.program().unwrap()));

            let transformation_location = gl.get_uniform_location(*self.symbol.program().unwrap(), "transformation").unwrap();
            gl.uniform_matrix_4_f32_slice(Some(&transformation_location), false, &t);

            let mode_location = gl.get_uniform_location(*self.symbol.program().unwrap(), "mode").unwrap();
            gl.uniform_1_u32(Some(&mode_location), drawing_mode.code());

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

impl<G, S: Symbol<G>> Layer for StaticLayer<G, S> {
    fn draw(&mut self, target: &RenderTarget, position: &MapPosition) {
        if self.features.len() == 0 {
            return;
        }

        self.set_context(target.context());

        let (width, height) = target.get_dimensions();
        self.draw_with_context(self.context.as_ref().unwrap(), position, width, height, DrawingMode::Normal);
    }

    fn feature_at_point(&self, target: &RenderTarget, screen_position: [i32; 2], map_position: &MapPosition) -> Option<usize> {
        let mut position = map_position.clone();
        let (width, height) = target.get_dimensions();
        position.translate_px(-screen_position[0] + width as i32 / 2, screen_position[1] - height as i32 / 2);
        position.set_screen_size(1, 1);

        let virtual_context = target.get_virtual_context(1, 1);
        self.draw_with_context(virtual_context.gl(), &position, 1, 1, DrawingMode::Selection);
        unsafe {
            virtual_context.gl().finish();
        }

        let pixel_value = virtual_context.pixel_value();

        if pixel_value == 0 {
            None
        } else {
            Some((pixel_value - 1) as usize)
        }
    }
}

impl<G, S: Symbol<G>> Drop for StaticLayer<G, S> {
    fn drop(&mut self) {
        self.clean();
    }
}

pub enum DrawingMode {
    Normal,
    Selection,
}

impl DrawingMode {
    fn code(&self) -> u32 {
        match self {
            DrawingMode::Normal => 0,
            DrawingMode::Selection => 1,
        }
    }
}