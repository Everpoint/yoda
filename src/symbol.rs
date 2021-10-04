use crate::{Color, Point, Point3};
use glium::{Program, Display};

pub struct CircleSymbol {
    pub color: Color,
    pub size: f32,
    pub program: Option<Program>,
}

impl CircleSymbol {
    const VERTEX_SHADER: &'static str = r#"
        #version 330

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

    const FRAGMENT_SHADER: &'static str = r#"
        #version 330

        in vec4 frag_color;
        out vec4 FragColor;

        void main() {
            FragColor = frag_color;
        }
    "#;

    pub fn compile(&mut self, display: &Display) {
        if self.program.is_none() {
            self.program = Some(glium::Program::from_source(display, Self::VERTEX_SHADER, Self::FRAGMENT_SHADER, None).unwrap());
        }
    }

    pub fn program(&self) -> &Program {
        self.program.as_ref().unwrap()
    }

    pub fn convert(&self, point: &Point3) -> Vec<CirclePointVertex>{
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

        result
    }
}

#[derive(Copy, Clone)]
pub struct CirclePointVertex {
    position: Point3,
    direction: Point,
    color: Color,
    size: f32,
}

implement_vertex!(CirclePointVertex, position, direction, color, size);
