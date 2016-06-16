#[macro_use]
extern crate glium;

fn main() {
    use glium::{DisplayBuild, Surface};

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 786)
        .with_title(format!("Hello, World!"))
        .build_glium()
        .unwrap();

    loop {
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        frame.finish().expect("Succeeded drawing");

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
