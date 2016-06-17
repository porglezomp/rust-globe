#[macro_use]
extern crate glium;
extern crate cgmath;

use glium::{DisplayBuild, Surface, VertexBuffer, IndexBuffer, Program};
use glium::glutin::{WindowBuilder, Event};
use glium::index::PrimitiveType;
use cgmath::prelude::*;
use cgmath::{Matrix4, Quaternion, Vector3, PerspectiveFov, Rad, conv};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

fn main() {
    let display = WindowBuilder::new()
        .with_dimensions(1024, 786)
        .with_title("Hello, World!")
        .with_multisampling(8)
        .build_glium()
        .unwrap();

    let (shape, indices) = generate_sphere(14, 20);
    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
    let indices = IndexBuffer::new(&display, PrimitiveType::LinesList, &indices).unwrap();

    let vertex_shader_src = include_str!("shader.vert");
    let fragment_shader_src = include_str!("shader.frag");
    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let view: Matrix4<f32> = PerspectiveFov {
        fovy: Rad { s: 2.0 },
        aspect: 1024.0 / 786.0,
        near: 0.1,
        far: 10.0,
    }.into();
    let view = view * Matrix4::from_translation(Vector3::new(0.0, 0.0, -1.5));

    let mut orientation = Quaternion::<f32>::one();
    let mut drag = false;
    let (mut mouse_x, mut mouse_y) = (0, 0);
    loop {
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        frame.draw(&vertex_buffer, &indices, &program,
                   &uniform! {
                       matrix: conv::array4x4(Matrix4::from(orientation)),
                       view: conv::array4x4(view),
                   },
                   &Default::default()).unwrap();
        frame.finish().expect("Succeeded drawing");

        use glium::glutin::ElementState::{Pressed, Released};
        use glium::glutin::MouseButton;
        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                Event::MouseInput(Pressed, MouseButton::Left) => drag = true,
                Event::MouseInput(Released, MouseButton::Left) => drag = false,
                Event::MouseMoved(x, y) => {
                    if drag {
                        let (dx, dy) = (x - mouse_x, y - mouse_y);
                        let speed = 0.001;
                        orientation = orientation *
                            Quaternion::from_angle_y(Rad { s: dx as f32 * speed }) *
                            Quaternion::from_angle_x(Rad { s: dy as f32 * speed });
                    }
                    mouse_x = x;
                    mouse_y = y;
                }
                _ => (),
            }
        }
    }
}

fn generate_sphere(lat_count: u32, lon_count: u32) -> (Vec<Vertex>, Vec<u16>) {
    let mut shape = Vec::new();
    let mut indices = Vec::new();
    for lat in 0..lat_count {
        let vertical_angle = (lat+1) as f32 * std::f32::consts::PI / (lat_count+1) as f32;
        let z = vertical_angle.cos();
        let start_index = shape.len();
        for lon in 0..lon_count {
            let horizontal_angle = lon as f32 * std::f32::consts::PI * 2.0 / lon_count as f32;
            let x = horizontal_angle.cos() * vertical_angle.sin();
            let y = horizontal_angle.sin() * vertical_angle.sin();
            shape.push(Vertex { position: [x, y, z] });
            let index = start_index as u16 + lon as u16;
            indices.push(index);
            indices.push(if lon == lon_count - 1 {
                start_index as u16
            } else {
                index + 1
            });
        }
        if lat != lat_count - 1 {
            for lon in 0..lon_count {
                let base = start_index as u16 + lon as u16;
                indices.push(base);
                indices.push(base + lon_count as u16);
            }
        }
    }
    (shape, indices)
}
