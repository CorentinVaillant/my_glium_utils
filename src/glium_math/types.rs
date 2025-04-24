pub use my_rust_matrix_lib::my_matrix_lib::prelude::{Matrix, VectorMath};

pub type Vec2 = VectorMath<f32, 2>;
pub type Vec3 = VectorMath<f32, 3>;
pub type Vec4 = VectorMath<f32, 4>;

pub type DVec2 = VectorMath<f64, 2>;
pub type DVec3 = VectorMath<f64, 3>;
pub type DVec4 = VectorMath<f64, 4>;

pub type Mat2 = Matrix<f32, 2, 2>;
pub type Mat3 = Matrix<f32, 3, 3>;
pub type Mat4 = Matrix<f32, 4, 4>;

pub type DMat2 = Matrix<f64, 2, 2>;
pub type DMat3 = Matrix<f64, 3, 3>;
pub type DMat4 = Matrix<f64, 4, 4>;
