#![feature(globs)]
#![feature(unsafe_destructor)]

extern crate libc;
extern crate gl;
extern crate glutin;
extern crate native;

use gl::types::*;

mod screen;
mod program;

// Vertex data
static VERTEX_DATA: [GLfloat, ..6] = [
     0.0,  0.5,
     0.5, -0.5,
    -0.5, -0.5
];

// Vertex data
static VERTEX_DATA_2: [GLfloat, ..6] = [
     0.0, -0.5,
     0.5,  0.5,
    -0.5,  0.5
];

// Shader sources
static VS_SRC: &'static str =
   "#version 130
    in vec2 position;
    void main() {
       gl_Position = vec4(position, 0.0, 1.0);
    }";

static FS_SRC: &'static str =
   "#version 130
    out vec4 out_color;
    void main() {
       out_color = vec4(1.0, 1.0, 1.0, 1.0);
    }";

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let mut screen = screen::Screen::new(800, 600, "Window".to_string(), VS_SRC, FS_SRC);

    screen.add_geom("up triangle".to_string(), box VERTEX_DATA);
    screen.add_geom("down triangle".to_string(), box VERTEX_DATA_2);

    while !screen.should_close {
      screen.draw();
    }
}
