#[macro_use]
extern crate glium;
extern crate cgmath;

use glium::{DisplayBuild, Surface, VertexBuffer, IndexBuffer, Program, DrawParameters, Blend};
use glium::glutin::{WindowBuilder, Event};
use glium::index::PrimitiveType;
use cgmath::prelude::*;
use cgmath::{Matrix4, Quaternion, Vector3, PerspectiveFov, Rad, conv};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    order: f32,
}
implement_vertex!(Vertex, position, order);

fn init_view(w: u32, h: u32) -> Matrix4<f32> {
    Matrix4::from(PerspectiveFov {
        fovy: Rad { s: 2.0 },
        aspect: w as f32 / h as f32,
        near: 0.1,
        far: 10.0,
    }) * Matrix4::from_translation(Vector3::new(0.0, 0.0, -1.5))
}

fn main() {
    let display = WindowBuilder::new()
        .with_dimensions(1024, 786)
        .with_title("Hello, World!")
        .with_multisampling(8)
        .build_glium()
        .unwrap();

    let (shape, indices) = generate_sphere(8, 12);
    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
    let indices = IndexBuffer::new(&display, PrimitiveType::LinesList, &indices).unwrap();

    let vertex_shader_src = include_str!("shader.vert");
    let fragment_shader_src = include_str!("shader.frag");
    use glium::program::ProgramCreationError::CompilationError;
    let program = match Program::from_source(&display, vertex_shader_src, fragment_shader_src, None) {
        Ok(prog) => prog,
        Err(CompilationError(e)) => {
            println!("Compilation error: {}", e);
            return;
        }
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let mut view = init_view(1024, 786);

    let mut orientation = Quaternion::<f32>::one();
    let mut drag = false;
    let (mut mouse_x, mut mouse_y) = (0, 0);
    let mut time = 0.0f32;
    loop {
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.01, 0.0, 0.0);
        frame.draw(&vertex_buffer, &indices, &program,
                   &uniform! {
                       matrix: conv::array4x4(Matrix4::from(orientation)),
                       view: conv::array4x4(view),
                       time: time,
                   },
                   &DrawParameters {
                       blend: Blend::alpha_blending(),
                       line_width: Some(3.0),
                       .. Default::default()
                   }).unwrap();
        frame.finish().expect("Succeeded drawing");

        use glium::glutin::ElementState::{Pressed, Released};
        use glium::glutin::MouseButton;
        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                Event::MouseInput(Pressed, MouseButton::Left) => drag = true,
                Event::MouseInput(Released, MouseButton::Left) => {
                    drag = false;
                    time = 0.0;
                }
                Event::MouseMoved(x, y) => {
                    if drag {
                        time = 0.0;
                        let (dx, dy) = (x - mouse_x, y - mouse_y);
                        let speed = 0.001;
                        orientation = orientation *
                            Quaternion::from_angle_y(Rad { s: dx as f32 * speed }) *
                            Quaternion::from_angle_x(Rad { s: dy as f32 * speed });
                    }
                    mouse_x = x;
                    mouse_y = y;
                }
                Event::Resized(w, h) => {
                    view = init_view(w, h);
                    println!("Resized to {}x{}", w, h);
                }
                _ => (),
            }
        }

        if !drag {
            time += 0.1;
        }
    }
}

fn generate_sphere(lat_count: u32, lon_count: u32) -> (Vec<Vertex>, Vec<u16>) {
    let mut shape = Vec::new();
    let mut indices = Vec::new();

    if lat_count < 3 || lon_count < 3 {
        return (shape, indices);
    }

    let fac = std::f32::consts::PI * 2.0 / lon_count as f32;
    let mut order = 0.0;

    for lon in 0..lon_count {
        let start_index = shape.len();
        let horizontal_angle = lon as f32 * fac;
        let x = horizontal_angle.cos();
        let y = horizontal_angle.sin();
        for lat in 0..lat_count {
            let vertical_angle = (lat+1) as f32 * std::f32::consts::PI / (lat_count+1) as f32;
            let z = vertical_angle.cos();
            let r = vertical_angle.sin();
            let (x, y) = (x * r, y * r);
            shape.push(Vertex { position: [x, y, z], order: order });
            order += 1.0;

            if lat != lat_count - 1 {
                let index = start_index as u16 + lat as u16;
                indices.push(index);
                indices.push(index + 1);
            }
        }
    }

    for lat in 0..lat_count {
        let vertical_angle = (lat+1) as f32 * std::f32::consts::PI / (lat_count+1) as f32;
        let z = vertical_angle.cos();
        let r = vertical_angle.sin();

        let start_index = shape.len();
        let mut first = true;
        for lon in (0..lon_count).chain(Some(0)) {
            let horizontal_angle = lon as f32 * fac;
            let x = horizontal_angle.cos() * r;
            let y = horizontal_angle.sin() * r;
            shape.push(Vertex { position: [x, y, z], order: order });
            order += 1.0;

            if first || lon != 0 {
                let index = start_index as u16 + lon as u16;
                indices.push(index);
                indices.push(index + 1);
            }
            first = false;
        }
    }

    (shape, indices)
}
