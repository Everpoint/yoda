use crate::{Color, Polyline, Point3};
use crate::symbol::Symbol;
use glium::{Display, Program};
use lyon::tessellation::geometry_builder::simple_builder;
use lyon::tessellation::{VertexBuffers, StrokeVertexConstructor, FillTessellator, StrokeOptions, StrokeTessellator};
use lyon::lyon_tessellation::{BuffersBuilder, StrokeVertex};
use lyon::tessellation::path::builder::{PathBuilder, Build};
use lyon::math::point;
use glium::draw_parameters::PolygonMode::Line;

pub struct LineSymbol {
    pub width: f32,
    pub color: Color,

    pub program: Option<Program>,
}

const VERTEX_SHADER: &'static str = r#"
#version 330

layout (location = 0) in vec3 position;
layout (location = 2) in vec4 color;
layout (location = 3) in float size;

uniform mat4 transformation;
uniform vec2 screen_size;

out vec4 frag_color;

void main() {
    gl_Position = vec4(position.xyz, 1.0) * transformation;
    frag_color = color;
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 330

in vec4 frag_color;
out vec4 FragColor;

void main() {
    FragColor = frag_color;
}
"#;

struct VertexCtor {
    color: Color,
}

impl StrokeVertexConstructor<LineVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> LineVertex {
        let point = vertex.position();
        LineVertex {
            position: [point.x, point.y, 0.0],
            color: self.color,
        }
    }
}

impl Symbol<Polyline> for LineSymbol {
    type Vertex = LineVertex;

    fn compile(&mut self, display: &Display) {
        if self.program.is_none() {
            self.program = Some(glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap());
        }
    }

    fn program(&self) -> &Program {
        self.program.as_ref().unwrap()
    }

    fn convert(&self, geometry: &Polyline) -> (Vec<Self::Vertex>, Option<Vec<u32>>) {
        if geometry.len() < 2 {
            return (vec![], None);
        }

        let mut buffers: VertexBuffers<LineVertex, u32> = VertexBuffers::new();
        let mut geometry_builder = BuffersBuilder::new(&mut buffers, VertexCtor {color: self.color});
        let mut tessellator = StrokeTessellator::new();
        let options = StrokeOptions::default().with_line_width(self.width);
        let mut builder = tessellator.builder(&options, &mut geometry_builder);

        builder.begin(point(geometry[0][0], geometry[0][1]));
        for p in geometry.iter().skip(1) {
            builder.line_to(point(p[0], p[1]));
        }
        builder.end(false);

        builder.build();

        let VertexBuffers {vertices, indices} = buffers;
        (vertices, Some(indices))
    }
}

#[derive(Copy, Clone)]
pub struct LineVertex {
    position: Point3,
    color: Color,
}

implement_vertex!(LineVertex, position, color);
