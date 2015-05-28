#[macro_use]
extern crate gfx;
extern crate gfx_window_glfw;
extern crate glfw;

use glfw::{Action, Key, OpenGlProfileHint, WindowEvent, WindowHint, WindowMode};
use gfx::traits::{Stream, ToIndexSlice, ToSlice, FactoryExt};

gfx_vertex!( Vertex {
    a_Pos@ pos: [f32; 2],
    a_Color@ color: [f32; 3],
});

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(3, 2));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    let (mut window, events) = glfw
        .create_window(640, 480, "polygon-al", WindowMode::Windowed)
        .unwrap();
    window.set_key_polling(true);
    let (mut stream, mut device, mut factory) = gfx_window_glfw::init(window);

    let triangle_data = [
        Vertex { pos: [-1.0, -1.0], color: [1.0, 1.0, 0.0] },
        Vertex { pos: [ 1.0, -1.0], color: [1.0, 0.0, 1.0] },
        Vertex { pos: [ 0.0,  1.0], color: [0.0, 1.0, 1.0] },
    ];
    let mesh = factory.create_mesh(&triangle_data);
    let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);

    let program = {
        let vs = gfx::ShaderSource {
            glsl_150: Some(include_bytes!("basic.vert")),
            .. gfx::ShaderSource::empty()
        };
        let fs = gfx::ShaderSource {
            glsl_150: Some(include_bytes!("basic.frag")),
            .. gfx::ShaderSource::empty()
        };
        factory.link_program_source(vs, fs).unwrap()
    };
    let state = gfx::DrawState::new();

    while !stream.out.window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) =>
                    stream.out.window.set_should_close(true),
                _ => {},
            }
        }

        stream.clear(gfx::ClearData {
            color: [0.3, 0.3, 0.3, 1.0],
            depth: 1.0,
            stencil: 0,
        });
        stream.draw(&gfx::batch::bind(&state, &mesh, slice.clone(), &program, &None))
              .unwrap();
        stream.present(&mut device);
    }
}
