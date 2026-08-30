// Q-Code Quantum IDE - Core Runtime & Principal Pipeline Interface
// Full Dual-Language Interprocess Router (OpenQASM 3.0 & Microsoft Q# Ready)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod simulator;
mod parser;

use simulator::state::QuantumState;
use simulator::matrix::{get_hadamard_matrix, get_pauli_x_matrix};
use parser::lexer::Lexer;
use parser::ast::{Parser, ASTNode};

#[tauri::command]
async fn analyze_quantum_code(code: String) -> Result<String, String> {
    if code.is_empty() {
        return Err("Execution Failure: Code stream buffer empty.".into());
    }

    // Determine target dialect layout profiles dynamically
    let is_qsharp = code.contains("namespace") || code.contains("operation");
    println!("Q-Code Engine: Routing compilation context. Target Dialect: {}", if is_qsharp { "Microsoft Q#" } else { "OpenQASM 3.0" });

    // 1. DISPATCH ENHANCED PARSER ENGINE
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    
    let ast_tree = match parser.parse_program() {
        Ok(tree) => tree,
        Err(e) => return Err(format!("Compilation Aborted: {}", e)),
    };

    // 2. ORCHESTRATE PHYSICS PIPELINE FROM UNIFIED AST
    let mut quantum_system = QuantumState::new(2);

    for node in ast_tree.iter() {
        match node {
            ASTNode::QuantumGate { gate, target, .. } => {
                if gate == "H" {
                    let h_gate = get_hadamard_matrix();
                    quantum_system.apply_gate(0, &h_gate);
                } else if gate == "X" {
                    let x_gate = get_pauli_x_matrix();
                    quantum_system.apply_gate(0, &x_gate);
                }
                println!("Q-Code Physics: Routed quantum instruction [Gate: {}] down to hardware vector registers.", gate);
            }
            _ => {}
        }
    }

    let sample_amplitude_re = quantum_system.state_vector[0].re;
    let sample_amplitude_im = quantum_system.state_vector[0].im;

    Ok(format!(
        "{{\"status\": \"success\", \"dialect\": \"{}\", \"nodes_parsed\": {}, \"amplitude_00_re\": {}, \"amplitude_00_im\": {}}}",
        if is_qsharp { "QSharp" } else { "OpenQASM" },
        ast_tree.len(),
        sample_amplitude_re,
        sample_amplitude_im
    ))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![analyze_quantum_code])
        .run(tauri::generate_context!())
        .expect("Fatal Error: Quantum Infrastructure Execution crashed.");
}
