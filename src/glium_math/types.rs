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

pub fn as_uniform_mat2(m :Mat2)->glium::uniforms::UniformValue<'static>{
    glium::uniforms::UniformValue::Mat2([
        [m[0][0],m[0][1]],
        [m[1][0],m[1][1]],
    ])
}

pub fn as_uniform_dmat2(m :DMat2)->glium::uniforms::UniformValue<'static>{
    glium::uniforms::UniformValue::DoubleMat2([
        [m[0][0],m[0][1]],
        [m[1][0],m[1][1]],
    ])
}

pub fn as_uniform_mat3(m :Mat3)->glium::uniforms::UniformValue<'static>{
    glium::uniforms::UniformValue::Mat3([
        [m[0][0],m[0][1],m[0][2]],
        [m[1][0],m[1][1],m[1][2]],
        [m[2][0],m[2][1],m[2][2]],
    ])
}

pub fn as_uniform_dmat3(m :DMat3)->glium::uniforms::UniformValue<'static>{
    glium::uniforms::UniformValue::DoubleMat3([
        [m[0][0],m[0][1],m[0][2]],
        [m[1][0],m[1][1],m[1][2]],
        [m[2][0],m[2][1],m[2][2]],
    ])
}

pub fn as_uniform_mat4(m :Mat4)->glium::uniforms::UniformValue<'static>{
    glium::uniforms::UniformValue::Mat4([
        [m[0][0],m[0][1],m[0][2],m[0][3]],
        [m[1][0],m[1][1],m[1][2],m[1][3]],
        [m[2][0],m[2][1],m[2][2],m[2][3]],
        [m[3][0],m[3][1],m[3][2],m[3][3]],
    ])
}

pub fn as_uniform_dmat4(m :DMat4)->glium::uniforms::UniformValue<'static>{
    glium::uniforms::UniformValue::DoubleMat4([
        [m[0][0],m[0][1],m[0][2],m[0][3]],
        [m[1][0],m[1][1],m[1][2],m[1][3]],
        [m[2][0],m[2][1],m[2][2],m[2][3]],
        [m[3][0],m[3][1],m[3][2],m[3][3]],
    ])
}