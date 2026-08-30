// Q-Code IDE - Quantum Core Physics Simulator
// High-performance linear algebra for quantum state transitions

use ndarray::{Array2, Axis};
use std::num::Wrapping;

// Representation of a Complex Number for Quantum Amplitudes
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    pub fn one() -> Self {
        Self { re: 1.0, im: 0.0 }
    }

    pub fn add(self, other: Self) -> Self {
        Self::new(self.re + other.re, self.im + other.im)
    }

    pub fn mul(self, other: Self) -> Self {
        Self::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}

// Generates the standard 2x2 Hadamard Matrix for Superposition
pub fn get_hadamard_matrix() -> Vec<Vec<Complex>> {
    let inv_sqrt2 = 1.0 / 2.0f64.sqrt();
    vec![
        vec![Complex::new(inv_sqrt2, 0.0), Complex::new(inv_sqrt2, 0.0)],
        vec![Complex::new(inv_sqrt2, 0.0), Complex::new(-inv_sqrt2, 0.0)],
    ]
}

// Generates the standard 2x2 Pauli-X (NOT) Matrix for Bit-Flips
pub fn get_pauli_x_matrix() -> Vec<Vec<Complex>> {
    vec![
        vec![Complex::zero(), Complex::one()],
        vec![Complex::one(), Complex::zero()],
    ]
}

// Computes the Kronecker (Tensor) Product to expand the quantum state space
pub fn kronecker_product(a: &[Vec<Complex>], b: &[Vec<Complex>]) -> Vec<Vec<Complex>> {
    let a_rows = a.len();
    let a_cols = a[0].len();
    let b_rows = b.len();
    let b_cols = b[0].len();

    let mut result = vec![vec![Complex::zero(); a_cols * b_cols]; a_rows * b_rows];

    for i in 0..a_rows {
        for j in 0..a_cols {
            for k in 0..b_rows {
                for l in 0..b_cols {
                    let row = i * b_rows + k;
                    let col = j * b_cols + l;
                    result[row][col] = result[row][col].add(a[i][j].mul(b[k][l]));
                }
            }
        }
    }
    result
}
