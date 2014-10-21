#![feature(phase)]

#[phase(plugin)] extern crate glium_macros;
extern crate glium;
extern crate glutin;

use glium::DisplayBuild;
use std::rand::{task_rng, Rng};

#[vertex_format]
struct Vertex {
  position: [f32, ..2],
  color: [f32, ..3],
}

#[uniforms]
struct Uniforms {
    matrix: [[f32, ..4], ..4],
}

fn main() {

  let display = glutin::WindowBuilder::new()
    .with_dimensions(800, 600)
    .with_title("Hello world".to_string())
    .build_glium().unwrap();

  let vertex_buffer = glium::VertexBuffer::new(&display, vec![
    Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
    Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
  ]);

  let index_buffer = glium::IndexBuffer::new(&display, glium::TrianglesList, &[ 0u16, 1, 2 ]);

  let program = glium::Program::new(&display,
    // vertex shader
    "   #version 110

    uniform mat4 matrix;

    attribute vec2 position;
    attribute vec3 color;

    varying vec3 v_color;

    void main() {
      gl_Position = vec4(position, 0.0, 1.0) * matrix;
      v_color = color;
    }
    ",

    // fragment shader
    "   #version 110
    varying vec3 v_color;

    void main() {
      gl_FragColor = vec4(v_color, 1.0);
    }
    ",

    // optional geometry shader
    None
  ).unwrap();

  let uniforms = Uniforms {
      matrix: [
          [ 1.0, 0.0, 0.0, 0.0 ],
          [ 0.0, 1.0, 0.0, 0.0 ],
          [ 0.0, 0.0, 1.0, 0.0 ],
          [ 0.0, 0.0, 0.0, 1.0 ]
      ],
  };

  loop {
    let mut target = display.draw();
    // target.clear_color(
    //   task_rng().gen_range(0.0, 1.0),
    //   task_rng().gen_range(0.0, 1.0),
    //   task_rng().gen_range(0.0, 1.0),
    //   0.0
    // );
    target.clear_color(0.0, 0.0, 0.0, 0.0);
    target.draw(glium::BasicDraw(&vertex_buffer, &index_buffer, &program, &uniforms, &std::default::Default::default()));
    target.finish();
  }
}
