// Q-Code IDE - Quantum State Vector Management Engine

use super::matrix::{Complex, get_hadamard_matrix, kronecker_product};

pub struct QuantumState {
    pub num_qubits: usize,
    pub state_vector: Vec<Complex>,
}

impl QuantumState {
    // Initializes the system in the ground state |00...0>
    pub fn new(num_qubits: usize) -> Self {
        let size = 1 << num_qubits; // 2^n state space
        let mut state = vec![Complex::zero(); size];
        state[0] = Complex::one(); // System starts at absolute |0>

        Self {
            num_qubits,
            state_vector: state,
        }
    }

    // Applies a single-qubit gate to a target qubit using tensor product scaling
    pub fn apply_gate(&mut self, target_qubit: usize, gate_matrix: &[Vec<Complex>]) {
        let mut full_operator = vec![vec![Complex::one()]]; // Identity starter

        for i in 0..self.num_qubits {
            if i == target_qubit {
                full_operator = kronecker_product(&full_operator, gate_matrix);
            } else {
                // Identity Matrix for non-targeted qubits
                let identity = vec![
                    vec![Complex::one(), Complex::zero()],
                    vec![Complex::zero(), Complex::one()],
                ];
                full_operator = kronecker_product(&full_operator, &identity);
            }
        }

        // Multiply the full operator matrix with the state vector
        let mut next_state = vec![Complex::zero(); self.state_vector.len()];
        for i in 0..full_operator.len() {
            let mut sum = Complex::zero();
            for j in 0..full_operator[i].len() {
                sum = sum.add(full_operator[i][j].mul(self.state_vector[j]));
            }
            next_state[i] = sum;
        }
        self.state_vector = next_state;
    }
}
