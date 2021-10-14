use crate::{Color, Polygon};
use crate::symbol::Symbol;
use lyon::tessellation::{VertexBuffers, FillTessellator, FillOptions, FillRule, FillVertexConstructor, FillVertex, StrokeTessellator, StrokeOptions, StrokeBuilder, FillBuilder};
use lyon::lyon_tessellation::{BuffersBuilder,StrokeVertex};
use lyon::tessellation::path::builder::{PathBuilder, Build};
use lyon::math::point;
use glow::Program;
use crate::symbol::line::{LineVertex, VertexCtor};

pub struct PolygonSymbol {
    pub fill_color: Color,
    pub stroke_width: f32,
    pub stroke_color: Color,
    pub program: Option<Program>
}

const VERTEX_SHADER: &'static str = r#"
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

const FRAGMENT_SHADER: &'static str = r#"
precision mediump float;

in vec4 frag_color;
out vec4 FragColor;

void main() {
    FragColor = frag_color;
}
"#;

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
        let mut fill_builder = tessellator.builder(&options, &mut geometry_builder);

        for contour in geometry {
            fill_builder.begin(point(contour[0][0], contour[0][1]));
            for p in contour.iter().skip(1) {
                fill_builder.line_to(point(p[0], p[1]));
            }
            fill_builder.end(true);
        }

        fill_builder.build().unwrap();

        let mut geometry_builder = BuffersBuilder::new(&mut buffers, VertexCtor {color: self.stroke_color});
        let mut tessellator = StrokeTessellator::new();
        let options = StrokeOptions::default().with_line_width(self.stroke_width);
        let mut stroke_builder = tessellator.builder(&options, &mut geometry_builder);

        for contour in geometry {
            stroke_builder.begin(point(contour[0][0], contour[0][1]));
            for p in contour.iter().skip(1) {
                stroke_builder.line_to(point(p[0], p[1]));
            }
            stroke_builder.end(true);
        }

        stroke_builder.build().unwrap();

        let VertexBuffers {vertices, indices} = buffers;
        (vertices, Some(indices))
    }
}
