extern crate gl;

use glutin::Window;

pub struct Screen {
  pub width: u16,
  pub height: u16,
  pub should_close: bool,
  window: Window
}

impl Screen {
  pub fn new(width: u16, height: u16, title: &str) -> Screen {
    let window = Window::new().unwrap();
    window.set_title(title);
    unsafe { window.make_current() };
    // Load the OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s));

    Screen {
      width: width,
      height: height,
      window: window,
      should_close: false
    }
  }

  pub fn draw(&self) {
    // Poll events
    self.window.poll_events();

    // Clear the screen to black
    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);

    // Draw a triangle from the 3 vertices
    gl::DrawArrays(gl::TRIANGLES, 0, 3);

    // Swap buffers
    self.window.swap_buffers();
  }
}
