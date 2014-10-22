extern crate gl;

use gl::types::*;
use glutin::{Window, WindowBuilder};
use std::collections::HashMap;
use std::mem;
use std::ptr;

use program::Program;

struct Geometry<'g> {
  vao: u32,
  vbo: u32
}

pub struct Screen<'s> {
  pub should_close: bool,
  window: Window,
  program: Program,
  geometry: Box<HashMap<String, Geometry<'s>>>
}

impl<'s> Screen<'s> {
  pub fn new<'s>(width: uint, height: uint, title: String, vertex_shader: &str, fragment_shader: &str) -> Screen<'s> {
    let window = WindowBuilder::new()
      .with_dimensions(width, height)
      .with_title(title)
      .build().unwrap();
    unsafe { window.make_current() };
    // Load the OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s));

    let program = Program::new(vertex_shader, fragment_shader);

    Screen {
      window: window,
      program: program,
      should_close: false,
      geometry: box HashMap::new()
    }
  }

  pub fn add_geom(&mut self, label: String, vertices: Box<[GLfloat]>) {
    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&vertices[0]),
                       gl::STATIC_DRAW);

        // Use shader program
        gl::UseProgram(self.program.id);
        "out_color".with_c_str(|ptr| gl::BindFragDataLocation(self.program.id, 0, ptr));

        // Specify the layout of the vertex data
        let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(self.program.id, ptr));
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                gl::FALSE as GLboolean, 0, ptr::null());
    }
    self.geometry.insert(label, Geometry {
      vao: vao,
      vbo: vbo
    });
  }

  pub fn draw(&self) {
    self.window.poll_events();

    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);

    for g in self.geometry.values() {
      gl::BindVertexArray(g.vao);
      gl::BindBuffer(gl::ARRAY_BUFFER, g.vbo);
      gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }

    self.window.swap_buffers();
  }
}

#[unsafe_destructor]
impl<'s> Drop for Screen<'s> {
  fn drop(&mut self) {
    for g in self.geometry.values() {
      unsafe {
          gl::DeleteBuffers(1, &g.vbo);
          gl::DeleteVertexArrays(1, &g.vao);
      }
    }
  }
}
