use ark_bls12_381::Fr;
use ark_std::{UniformRand};
use nalgebra::{DMatrix};

use std::error;
//use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn powers(x: Fr, n: usize) -> nalgebra::DVector<Fr> {
    let mut value = x;
    let mut pows = Vec::new();
    for _exponent in 1..(n+1) {
	pows.push(value);
	value *= x;
    }
    pows.into()
}

fn random_powers(n: usize) -> nalgebra::DVector<Fr> {
    let mut rng = ark_std::rand::thread_rng();
    let x = Fr::rand(&mut rng);
    powers(x,n)
}

/// C is the claimed product of A and B
fn verify(A: DMatrix<Fr>, B: DMatrix<Fr>, C: DMatrix<Fr>) -> Result<bool> {
    let (a_rows, a_cols) = A.shape();
    let (b_rows, b_cols) = B.shape();
    let (c_rows, c_cols) = C.shape();
    assert!(a_rows == c_rows && a_cols == b_rows && b_cols == c_cols, "Mismatched dimensions.");

    let x = random_powers(c_cols);
    let rhs = C*x.clone();
    let lhs = A*(B*x);
    
    Ok(rhs == lhs)
}

pub fn test() -> bool {
    let a = DMatrix::from_vec(2,3, vec![1u64.into(),2u64.into(),3u64.into(),4u64.into(),5u64.into(),6u64.into()]);
    let b = a.clone().transpose();
    let c = a.clone()*b.clone();
    verify(a,b,c).unwrap()
}

