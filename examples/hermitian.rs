extern crate nalgebra as na;
extern crate num_complex;

use na::{Matrix2, Matrix4};
use na::SymmetricEigen;
use num_complex::Complex64 as c64;

fn main(){
    let _1c = c64::new(1.0, 0.0);
    let _0c = c64::new(0.0, 0.0);
    let _i = c64::i();

    let sx = Matrix2::new(
        _0c, _1c,
        _1c, _0c      );

    let sy = Matrix2::new(
        _0c, -_i,
        _i,  _0c
    );

    let sz = Matrix2::new(
        _1c, _0c,
        _0c, -_1c
    );

    let h1 : Matrix2<c64> = sz + sy;
    let eig  = SymmetricEigen::new(h1.clone());
    println!("Matrix:\n{}", h1);
    println!("Eigenvalues:\n{}", eig.eigenvalues);
    println!("Eigenvectors:\n{}", eig.eigenvectors);


    let h2 = Matrix4::<c64>::new(
        _0c, _0c,  _0c, -_i,
        _0c, _0c,  _i,  _0c,
        _0c, -_i,  _0c, _0c,
        _i,  _0c,  _0c, _0c
    );

    let eig = SymmetricEigen::new(h2.clone());
    println!("Matrix:\n{}", h2);
    println!("Eigenvalues:\n{}", eig.eigenvalues);
    println!("Eigenvectors:\n{}", eig.eigenvectors);

}