use crate::gl::{AttributeValueType, Vertex, VertexAttribute};
use crate::symbol::Symbol;
use crate::{Color, Point3, Polyline};
use glow::Program;
use lyon::lyon_tessellation::{BuffersBuilder, StrokeVertex};
use lyon::math::point;
use lyon::tessellation::path::builder::{Build, PathBuilder};
use lyon::tessellation::{
    StrokeOptions, StrokeTessellator, StrokeVertexConstructor, VertexBuffers,
};

pub struct LineSymbol {
    pub width: f32,
    pub color: Color,

    pub program: Option<Program>,
}

const VERTEX_SHADER: &str = r#"
layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;
layout (location = 2) in uint id;

uniform mat4 transformation;
uniform uint mode;
uniform vec2 screen_size;

out vec4 frag_color;

void main() {
    gl_Position = vec4(position.xyz, 1.0) * transformation;
    frag_color = color;
    if (mode == 0u) {
        frag_color = color;
    }
    if (mode == 1u) {
        frag_color = vec4((float(id) + 1.0) / 255.0, 0.0, 0.0, 1.0);
    }
}
"#;

const FRAGMENT_SHADER: &str = r#"
precision mediump float;

in vec4 frag_color;
out vec4 FragColor;

void main() {
    FragColor = frag_color;
}
"#;

pub struct VertexCtor {
    pub color: Color,
    pub id: u32,
}

impl StrokeVertexConstructor<LineVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> LineVertex {
        let point = vertex.position();
        LineVertex {
            position: [point.x, point.y, 0.0],
            color: self.color,
            id: self.id,
        }
    }
}

impl Symbol<Polyline> for LineSymbol {
    type Vertex = LineVertex;

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

    fn convert(&self, geometry: &Polyline, id: u32) -> (Vec<Self::Vertex>, Option<Vec<u32>>) {
        if geometry.len() < 2 {
            return (vec![], None);
        }

        let mut buffers: VertexBuffers<LineVertex, u32> = VertexBuffers::new();
        let mut geometry_builder = BuffersBuilder::new(
            &mut buffers,
            VertexCtor {
                color: self.color,
                id,
            },
        );
        let mut tessellator = StrokeTessellator::new();
        let options = StrokeOptions::default().with_line_width(self.width);
        let mut builder = tessellator.builder(&options, &mut geometry_builder);

        builder.begin(point(geometry[0][0], geometry[0][1]));
        for p in geometry.iter().skip(1) {
            builder.line_to(point(p[0], p[1]));
        }
        builder.end(false);

        builder.build().unwrap();

        let VertexBuffers { vertices, indices } = buffers;
        (vertices, Some(indices))
    }
}

#[derive(Copy, Clone)]
pub struct LineVertex {
    pub position: Point3,
    pub color: Color,
    pub id: u32,
}

impl Vertex for LineVertex {
    fn attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute {
                location: 0,
                size: 3,
                value_type: AttributeValueType::Float,
            },
            VertexAttribute {
                location: 1,
                size: 4,
                value_type: AttributeValueType::Float,
            },
            VertexAttribute {
                location: 2,
                size: 1,
                value_type: AttributeValueType::UnsignedInteger,
            },
        ]
    }
}
