use crate::{Color, Point, Point3};
use crate::symbol::Symbol;
use glow::{Context, HasContext, Program};
use std::hash::Hash;

pub struct CircleSymbol {
    pub color: Color,
    pub size: f32,
    pub program: Option<Program>,
}

const VERTEX_SHADER: &'static str = r#"#version 300 es

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 direction;
layout (location = 2) in vec4 color;
layout (location = 3) in float size;

uniform mat4 transformation;
uniform vec2 screen_size;

out vec4 frag_color;

void main() {
    vec2 dir = direction * size / screen_size;
    gl_Position = vec4((vec4(position.xyz, 1.0) * transformation + vec4(dir, 0.0, 0.0)).xy, 0.0, 1.0);
    frag_color = color;
}
"#;

const FRAGMENT_SHADER: &'static str = r#"#version 300 es

precision mediump float;

in vec4 frag_color ;
out vec4 FragColor;

void main() {
    FragColor = frag_color;
}
"#;

impl Symbol<Point3> for CircleSymbol {
    type Vertex = CirclePointVertex;

    fn vertex_shader(&self) -> &str {
        VERTEX_SHADER
    }

    fn fragment_shader(&self) -> &str {
        FRAGMENT_SHADER
    }

    fn compile(&mut self, gl: &Context) {
        if self.program.is_none() {
            unsafe {
                let program = gl.create_program().unwrap();

                let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
                gl.shader_source(vertex_shader, self.vertex_shader());
                gl.compile_shader(vertex_shader);
                if !gl.get_shader_compile_status(vertex_shader) {
                    panic!("Failed to compile vertex shader: {}", gl.get_shader_info_log(vertex_shader));
                }

                gl.attach_shader(program, vertex_shader);

                let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
                gl.shader_source(fragment_shader, self.fragment_shader());
                gl.compile_shader(fragment_shader);
                if !gl.get_shader_compile_status(fragment_shader) {
                    panic!("Failed to compile fragment shader: {}", gl.get_shader_info_log(fragment_shader));
                }

                gl.attach_shader(program, fragment_shader);

                gl.link_program(program);
                if !gl.get_program_link_status(program) {
                    panic!("Failed to link program: {}", gl.get_program_info_log(program));
                }

                gl.detach_shader(program, vertex_shader);
                gl.delete_shader(vertex_shader);
                gl.detach_shader(program, fragment_shader);
                gl.delete_shader(fragment_shader);

                self.program = Some(program);
            }
        }
    }

    fn program(&self) -> &(Program) {
        self.program.as_ref().unwrap()
    }

    fn convert(&self, point: &Point3) -> (Vec<Self::Vertex>, Option<Vec<u32>>) {
        let mut result = vec![];
        const segments: usize = 16;
        for i in 0..segments {
            result.push(CirclePointVertex {position: point.clone(), direction: [0.0, 0.0], size: self.size, color: self.color});

            let from = (i as f32) / (segments as f32);

            let angle = std::f32::consts::PI * from * 2.0;

            let dx = angle.cos();
            let dy = angle.sin();

            result.push(CirclePointVertex {position: point.clone(), direction: [dx, dy], size: self.size, color: self.color});

            let to = (i as f32 + 1.0) / (segments as f32);
            let angle = std::f32::consts::PI * to * 2.0;

            let dx = angle.cos();
            let dy = angle.sin();

            result.push(CirclePointVertex {position: point.clone(), direction: [dx, dy], size: self.size, color: self.color});
        }

        (result, None)
    }
}

#[derive(Copy, Clone)]
pub struct CirclePointVertex {
    position: Point3,
    direction: Point,
    color: Color,
    size: f32,
}

unsafe fn to_bytes<T>(p: &T, size: usize) -> &[u8] {
    std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        size,
    )
}
