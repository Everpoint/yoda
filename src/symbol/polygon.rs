use crate::{Color, Polyline, Point3, Polygon};
use crate::symbol::Symbol;
use lyon::tessellation::geometry_builder::simple_builder;
use lyon::tessellation::{VertexBuffers, StrokeVertexConstructor, FillTessellator, StrokeOptions, StrokeTessellator, FillOptions, FillRule, FillVertexConstructor, FillVertex};
use lyon::lyon_tessellation::{BuffersBuilder, StrokeVertex};
use lyon::tessellation::path::builder::{PathBuilder, Build};
use lyon::math::point;
use glow::{Context, Program};
use crate::symbol::line::LineVertex;

pub struct PolygonSymbol {
    pub fill_color: Color,

    pub program: Option<Program>,
}

const VERTEX_SHADER: &'static str = r#"#version 300 es

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;

uniform mat4 transformation;
uniform vec2 screen_size;

out vec4 frag_color;

void main() {
    gl_Position = vec4(position.xyz, 1.0) * transformation;
    frag_color = color;
}
"#;

const FRAGMENT_SHADER: &'static str = r#"#version 300 es

precision mediump float;

in vec4 frag_color;
out vec4 FragColor;

void main() {
    FragColor = frag_color;
}
"#;

struct VertexCtor {
    color: Color,
}

impl FillVertexConstructor<LineVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: FillVertex) -> LineVertex {
        let point = vertex.position();
        LineVertex {
            position: [point.x, point.y, 0.0],
            color: self.color,
        }
    }
}

impl Symbol<Polygon> for PolygonSymbol {
    type Vertex = LineVertex;

    fn vertex_shader(&self) -> &str {
        VERTEX_SHADER
    }

    fn fragment_shader(&self) -> &str {
        FRAGMENT_SHADER
    }

    fn set_program(&mut self, program: Program) {
        self.program = Some(program)
    }

    fn program(&self) -> Option<&Program> {
        self.program.as_ref()
    }

    fn convert(&self, geometry: &Polygon) -> (Vec<Self::Vertex>, Option<Vec<u32>>) {
        let mut buffers: VertexBuffers<LineVertex, u32> = VertexBuffers::new();
        let mut geometry_builder = BuffersBuilder::new(&mut buffers, VertexCtor {color: self.fill_color });
        let mut tessellator = FillTessellator::new();
        let options = FillOptions::default().with_fill_rule(FillRule::EvenOdd);
        let mut builder = tessellator.builder(&options, &mut geometry_builder);

        for contour in geometry {
            builder.begin(point(contour[0][0], contour[0][1]));
            for p in contour.iter().skip(1) {
                builder.line_to(point(p[0], p[1]));
            }
            builder.end(true);
        }

        builder.build();

        let VertexBuffers {vertices, indices} = buffers;
        (vertices, Some(indices))
    }
}
