#![cfg(test)]

use crate::glium_math::Mat4;

#[test]
fn test_mat(){
    let a = Mat4::from([
        [1.,2.,3.,4.],
        [5.,6.,7.,8.],
        [9.,10.,11.,12.,],
        [13.,14.,15.,16.,],
    ]);

    let b = Mat4::from_fn(|i,j|{i as f32 * 10. + j as f32});


    let _ = a+b;
}