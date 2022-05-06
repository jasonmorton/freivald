use ark_bls12_381::Fr as F;
use ark_ff::BigInteger256;
use ark_std::UniformRand;
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use reqwasm;
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

/// C is the claimed product of A and B
fn verify(a: DMatrix<F>, b: DMatrix<F>, c: DMatrix<F>) -> bool {
    let (a_rows, a_cols) = a.shape();
    let (b_rows, b_cols) = b.shape();
    let (c_rows, c_cols) = c.shape();
    assert!(
        a_rows == c_rows && a_cols == b_rows && b_cols == c_cols,
        "Mismatched dimensions."
    );

    let x = random_powers(c_cols);
    let rhs = c * x.clone();
    let lhs = a * (b * x);

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

pub fn to_bytes(m: DMatrix<F>) -> DMatrix<[u64; 4]> {
    m.map(|alpha| alpha.0 .0)
}
pub fn from_bytes(m: DMatrix<[u64; 4]>) -> DMatrix<F> {
    m.map(|bytes| F::from(ark_ff::BigInteger256(bytes)))
}

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub a: DMatrix<[u64; 4]>,
    pub b: DMatrix<[u64; 4]>,
}

#[derive(Serialize, Deserialize)]
pub struct Solution {
    pub c: DMatrix<[u64; 4]>,
}

pub async fn fetch_multiplication_u64(instance: Instance) -> Solution {
    let req = reqwasm::http::Request::new("http://localhost:8000/mul");
    let req = req.body(JsValue::from_serde(&instance).unwrap()); //trait `From<Instance>` is not implemented for `wasm_bindgen::JsValue`
                                                                 // body: impl Into<JsValue>

    let resp = req.send().await.unwrap();
    let s: Solution = resp.json().await.unwrap(); //T: serde::de::DeserializeOwned
    s
}

pub async fn remote_mul(a: DMatrix<F>, b: DMatrix<F>) -> DMatrix<F> {
    let instance = Instance {
        a: to_bytes(a),
        b: to_bytes(b),
    };
    let solution = fetch_multiplication_u64(instance).await;
    from_bytes(solution.c)
}
