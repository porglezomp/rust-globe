#[macro_use]
extern crate glium;
extern crate cgmath;

use glium::{DisplayBuild, Surface, VertexBuffer, IndexBuffer, Program};
use glium::glutin::{WindowBuilder, Event};
use glium::index::PrimitiveType;
use cgmath::{Matrix4, Vector3, conv};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let display = WindowBuilder::new()
        .with_dimensions(1024, 786)
        .with_title("Hello, World!")
        .build_glium()
        .unwrap();

    let shape = [
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [-1.0, 1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [1.0, -1.0] },
    ];
    let indices = [
        0u8, 1, 2,
        2, 3, 0,
    ];
    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
    let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices).unwrap();

    let vertex_shader_src = include_str!("shader.vert");
    let fragment_shader_src = include_str!("shader.frag");
    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let matrix = Matrix4::from_translation(Vector3::new(0.5, 0.3, 0.2)) * Matrix4::from_scale(0.5f32);
    loop {
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        frame.draw(&vertex_buffer, &indices, &program, &uniform! { matrix: conv::array4x4(matrix) },
                   &Default::default()).unwrap();
        frame.finish().expect("Succeeded drawing");

        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                _ => (),
            }
        }
    }
}
