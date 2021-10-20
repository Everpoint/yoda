mod circle;
pub use circle::*;

mod line;
pub use line::LineSymbol;

mod polygon;
pub use polygon::PolygonSymbol;

use crate::gl::Vertex;
use glow::{Context, HasContext, Program};

#[cfg(not(target_arch = "wasm32"))]
const GL_VERSION: &str = r#"#version 330"#;

#[cfg(target_arch = "wasm32")]
const GL_VERSION: &str = r#"#version 300 es"#;

pub trait Symbol<G> {
    type Vertex: Vertex;

    fn vertex_shader(&self) -> &str;
    fn fragment_shader(&self) -> &str;

    fn compile(&mut self, gl: &Context) {
        if !self.is_compiled() {
            unsafe {
                let program = gl.create_program().unwrap();

                let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
                gl.shader_source(vertex_shader, &get_vertex_source(self.vertex_shader()));
                gl.compile_shader(vertex_shader);
                assert!(
                    gl.get_shader_compile_status(vertex_shader),
                    "Failed to compile vertex shader: {}",
                    gl.get_shader_info_log(vertex_shader)
                );

                gl.attach_shader(program, vertex_shader);

                let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
                gl.shader_source(fragment_shader, &get_vertex_source(self.fragment_shader()));
                gl.compile_shader(fragment_shader);
                assert!(
                    gl.get_shader_compile_status(fragment_shader),
                    "Failed to compile fragment shader: {}",
                    gl.get_shader_info_log(fragment_shader)
                );

                gl.attach_shader(program, fragment_shader);

                gl.link_program(program);
                assert!(
                    gl.get_program_link_status(program),
                    "Failed to link program: {}",
                    gl.get_program_info_log(program)
                );

                gl.detach_shader(program, vertex_shader);
                gl.delete_shader(vertex_shader);
                gl.detach_shader(program, fragment_shader);
                gl.delete_shader(fragment_shader);

                self.set_program(program);
            }
        }
    }

    fn set_program(&mut self, program: Program);

    fn is_compiled(&self) -> bool {
        self.program().is_some()
    }
    fn program(&self) -> Option<&Program>;
    fn convert(&self, geometry: &G, id: u32) -> (Vec<Self::Vertex>, Option<Vec<u32>>);
}

fn get_vertex_source(source: &str) -> String {
    format!("{}\n{}", GL_VERSION, source)
}
