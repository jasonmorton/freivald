use ark_bls12_381::Fr as F;
use ark_std::UniformRand;
use nalgebra::DMatrix;

use std::error;

fn powers(x: F, n: usize) -> nalgebra::DVector<F> {
    let mut value = x;
    let mut pows = Vec::new();
    for _exponent in 1..(n + 1) {
        pows.push(value);
        value *= x;
    }
    pows.into()
}

fn random_powers(n: usize) -> nalgebra::DVector<F> {
    let mut rng = ark_std::rand::thread_rng();
    let x = F::rand(&mut rng);
    powers(x, n)
}

/// Generate random A and B in F
pub fn generate_instance(input: usize, middle: usize, output: usize) -> (DMatrix<F>, DMatrix<F>) {
    let mut rng = ark_std::rand::thread_rng();
    let a_entries: Vec<F> = (0..(output * middle)).map(|_| F::rand(&mut rng)).collect();
    let a = DMatrix::from_vec(output, middle, a_entries);
    let b_entries: Vec<F> = (0..(input * middle)).map(|_| F::rand(&mut rng)).collect();
    let b = DMatrix::from_vec(middle, input, b_entries);
    (a, b)
}

#[allow(non_snake_case)]
/// C is the claimed product of A and B
fn verify(A: DMatrix<F>, B: DMatrix<F>, C: DMatrix<F>) -> bool {
    let (a_rows, a_cols) = A.shape();
    let (b_rows, b_cols) = B.shape();
    let (c_rows, c_cols) = C.shape();
    assert!(
        a_rows == c_rows && a_cols == b_rows && b_cols == c_cols,
        "Mismatched dimensions."
    );

    let x = random_powers(c_cols);
    let rhs = C * x.clone();
    let lhs = A * (B * x);

    rhs == lhs
}

pub fn perhapsverify(a: DMatrix<F>, b: DMatrix<F>, mc: Option<DMatrix<F>>) -> String {
    if let Some(c) = mc {
        if verify(a, b, c) {
            "Verified".to_string()
        } else {
            "Verification failed".to_string()
        }
    } else {
        "C not computed".to_string()
    }
}

