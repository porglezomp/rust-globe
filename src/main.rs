#[macro_use]
extern crate glium;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    use glium::{DisplayBuild, Surface};

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 786)
        .with_title(format!("Hello, World!"))
        .build_glium()
        .unwrap();

    let shape = vec![
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [-1.0, 1.0] },
        Vertex { position: [1.0, 1.0] },
        // Vertex { position: [1.0, -1.0] },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = include_str!("vertex.glsl");
    let fragment_shader_src = include_str!("fragment.glsl");
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    loop {
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                   &Default::default()).unwrap();
        frame.finish().expect("Succeeded drawing");

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
