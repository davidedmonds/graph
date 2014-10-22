#![feature(globs)]
#![feature(unsafe_destructor)]

extern crate libc;
extern crate gl;
extern crate glutin;
extern crate native;

use gl::types::*;

pub mod screen;
mod program;
