use engine::*;

use glium;
use glium::{Surface};
use glium::index::PrimitiveType;

pub struct Renderer {
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    window_width: u32,
    window_height: u32
}

pub struct Rect {
    pub x : f32,
    pub y : f32,
    pub w : f32,
    pub h : f32,
}

impl Renderer {
    pub fn draw_quad(&mut self, target: &mut glium::Frame, rect: Rect) 
    {
        let uniforms = uniform! {
            matrix: [
                [2.0 * rect.w / self.window_width as f32, 0.0, 0.0, -1.0 + (2.0 * rect.x / self.window_width as f32)],
                [0.0, -2.0 * rect.h / self.window_height as f32, 0.0, 1.0 - (2.0 * rect.y / self.window_height as f32)],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniforms,
            &Default::default())
                .unwrap();
    }
}

pub fn new(engine: &mut Engine) -> Renderer {
    // Build the vertex buffer for the quad.
    let vertex_buffer = {
        implement_vertex!(Vertex, position, color);

        glium::VertexBuffer::new(&engine.display,
            &[
                Vertex { position: [0.0, 0.0], color: [0.0, 1.0, 0.0] },
                Vertex { position: [1.0, 0.0], color: [0.0, 0.0, 1.0] },
                Vertex { position: [1.0, 1.0], color: [1.0, 0.0, 0.0] },
                Vertex { position: [0.0, 1.0], color: [1.0, 1.0, 0.0] },
            ]
        ).unwrap()
    };

    // Build the index buffer.
    let index_buffer = glium::IndexBuffer::new(
        &engine.display, PrimitiveType::TrianglesList, 
        &[0, 1, 2, 2, 3, 0]).unwrap();


    let vertex_shader_src = r#"
        #version 140
        uniform mat4 matrix;
        in vec2 position;
        in vec3 color;
        out vec3 vColor;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0) * matrix;
            vColor = color;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec3 vColor;
        out vec4 color;
        void main() {
            color = vec4(vColor, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&engine.display, vertex_shader_src, fragment_shader_src, None).unwrap();

    return Renderer {
        program,
        vertex_buffer,
        index_buffer,
        window_width: engine.window_width,
        window_height: engine.window_height
    }
}
