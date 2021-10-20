use crate::{Color, Point, Point3};
use crate::symbol::Symbol;
use glow::Program;
use crate::gl::{Vertex, VertexAttribute, AttributeValueType};

pub struct CircleSymbol {
    pub color: Color,
    pub size: f32,
    pub program: Option<Program>,
}

const VERTEX_SHADER: &'static str = r#"
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 direction;
layout (location = 2) in vec4 color;
layout (location = 3) in float size;
layout (location = 4) in uint id;

uniform mat4 transformation;
uniform vec2 screen_size;
uniform uint mode;

out vec4 frag_color;

void main() {
    vec2 dir = direction * size / screen_size;
    gl_Position = vec4((vec4(position.xyz, 1.0) * transformation + vec4(dir, 0.0, 0.0)).xy, 0.0, 1.0);
    if (mode == 0u) {
        frag_color = color;
    }
    if (mode == 1u) {
        frag_color = vec4((float(id) + 1.0) / 255.0, 0.0, 0.0, 1.0);
    }
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
precision mediump float;

in vec4 frag_color;
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

    fn set_program(&mut self, program: Program) {
        self.program = Some(program);
    }

    fn program(&self) -> Option<&Program> {
        self.program.as_ref()
    }

    fn convert(&self, point: &Point3, id: u32) -> (Vec<Self::Vertex>, Option<Vec<u32>>) {
        let mut result = vec![];
        const SEGMENTS: usize = 16;

        for i in 0..SEGMENTS {
            result.push(CirclePointVertex {position: point.clone(), direction: [0.0, 0.0], size: self.size, color: self.color, id});

            let from = (i as f32) / (SEGMENTS as f32);

            let angle = std::f32::consts::PI * from * 2.0;

            let dx = angle.cos();
            let dy = angle.sin();

            result.push(CirclePointVertex {position: point.clone(), direction: [dx, dy], size: self.size, color: self.color, id});

            let to = (i as f32 + 1.0) / (SEGMENTS as f32);
            let angle = std::f32::consts::PI * to * 2.0;

            let dx = angle.cos();
            let dy = angle.sin();

            result.push(CirclePointVertex {position: point.clone(), direction: [dx, dy], size: self.size, color: self.color, id});
        }

        (result, None)
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct CirclePointVertex {
    position: Point3,
    direction: Point,
    color: Color,
    size: f32,
    id: u32,
}

impl Vertex for CirclePointVertex {
    fn attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute {location: 0, size: 3, value_type: AttributeValueType::Float},
            VertexAttribute {location: 1, size: 2, value_type: AttributeValueType::Float},
            VertexAttribute {location: 2, size: 4, value_type: AttributeValueType::Float},
            VertexAttribute {location: 3, size: 1, value_type: AttributeValueType::Float},
            VertexAttribute {location: 4, size: 1, value_type: AttributeValueType::UnsignedInteger},
        ]
    }
}