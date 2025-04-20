//! This library contains all my utilities for projects using the Glium crate.
//! Feel free to use it, and don’t hesitate to report any bugs you find,
//! or to ask me questions about how to use it.
//!
//! However, don’t expect regular updates or bug fixes — I work on this in my free time,
//! alongside my studies, which already take up a lot of my time.

pub mod canvas;
pub mod datastruct;
pub mod glium_math;
pub mod mesh;


pub mod math{
    pub use crate::glium_math::*;
}

pub mod linear_algebra{
    pub use my_rust_matrix_lib::my_matrix_lib::prelude::*;
}