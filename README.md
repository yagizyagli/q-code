# ⚛️ Q-Code IDE | Cross-Platform Quantum Engineering Environment

**Q-Code** is a next-generation, ultra-high-performance Quantum Integrated Development Environment (IDE) engineered specifically for quantum software engineers and space systems architects. Powered by a dual-engine architecture (**Tauri + Rust** for core mechanics, **TypeScript + Vite** for interface layers), it enables seamless real-time compilation and simulation of multi-dialect quantum programs without stalling the user interface.

---

## 🚀 Key Innovations & Core Features

- 🛸 **Dual-Dialect Native Parsing Engine:** Fully capable of concurrently evaluating structural **IBM OpenQASM 3.0** expressions and block-scoped **Microsoft Q# (Quantum Sharp)** operational parameters (`namespaces`, `operations`).
- 🔬 **Deterministic Quantum Simulator Core:** Low-level, zero-cost abstraction state vector simulation implemented natively in pure Rust via high-performance tensor expansions (\(2^n\) computational Hilbert state space transformations).
- 🪐 **Hardware-Accelerated Visual Telemetry:** Integrated real-time 3D Bloch Sphere state animation (via **Three.js / WebGL**) mapping specific qubit transition probability amplitudes (\(\vert{}\psi\rangle = \alpha\vert{}0\rangle + \beta\vert{}1\rangle\)).
- ⚡ **Asynchronous IPC Pipeline Architecture:** Strict decoupling between high-computational linear algebraic matrix operations and frontend graphics execution to guarantee sub-millisecond IDE response cycles.

---

## 📐 System & Compilation Architecture

The system mimics the multi-process design topology used by modern enterprise development suites like VS Code, maximizing memory management and execution routing constraints.

```text
                               ┌──────────────────────────────────────────┐
                               │            Q-CODE FRONTEND LAYER         │
                               │   (Monaco Editor + TypeScript + Vite)   │
                               └────────────────────┬─────────────────────┘
                                                    │
                                         Async IPC Command Bridge
                                         (Debounced Data Payload)
                                                    │
                                                    ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                     TAURI / RUST CORE BACKEND                                    │
│                                                                                                 │
│  ┌───────────────────────────────┐                  ┌────────────────────────────────────────┐  │
│  │    COMPILER / PARSER STACK    │                  │       QUANTUM PHYSICS SIMULATOR        │  │
│  │                               │                  │                                        │  │
│  │ ┌───────────────────────────┐ │                  │ ┌────────────────────────────────────┐ │  │
│  │ │ Multi-Language Lexer      │ │                  │ │ Complex Number Amplitude Multipliers│ │  │
│  │ └─────────────┬─────────────┘ │                  │ └──────────────────┬─────────────────┘ │  │
│  │               │ Token Stream  │                  │                    │                   │  │
│  │               ▼               │                  │                    ▼                   │  │
│  │ ┌───────────────────────────┐ │  AST Instruction │ ┌────────────────────────────────────┐ │  │
│  │ │ Abstract Syntax Tree (AST)├─┼─────────────────►│ │ Kronecker (Tensor) Product Engine  │ │  │
│  │ └───────────────────────────┘ │      Router      │ └──────────────────┬─────────────────┘ │  │
│  │                               │                  │                    │ State Mapping     │  │
│  │                               │                  │                    ▼                   │  │
│  │                               │                  │ ┌────────────────────────────────────┐ │  │
│  │                               │                  │ │ 2^n Hilbert State Vector Registry  │ │  │
│  │                               │                  │ └────────────────────────────────────┘ │  │
│  └───────────────────────────────┘                  └────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 📁 Repository Directory Structure

```text
q-code/
├── src/                       ◄── FRONTEND UI RESIDENCE (Web & WebGL Layer)
│   ├── main.ts                ◄── Main System Pipeline & Debounced IPC Bridge Dispatcher
│   ├── style.css              ◄── Immersive Dark Cyberpunk Cyber-Theme Layout Styling
│   └── visualizer/
│       ├── bloch.ts           ◄── 3D Quantum Bloch Sphere Vector Kinematics (Three.js Engine)
│       └── circuit.ts         ◄── High-Fidelity HTML5 Canvas Quantum Line Wire Matrix Renderer
└── src-tauri/                 ◄── LOW-LEVEL BACKEND COMPUTATION LAYER (Rust Engine)
    ├── Cargo.toml             ◄── Structural Cargo Blueprint (ndarray, serde_json, lazy_static)
    ├── tauri.conf.json        ◄── System Window Constraints & IPC Security Whitelist Directives
    └── src/
        ├── main.rs            ◄── Command Orchestrator & Multi-Language Router Interface
        ├── parser/            ◄── Compiler Subsystem (Lexing & AST Matrix Production Trees)
        └── simulator/         ◄── Core Physical Simulator (Hermitian & Kronecker Operators)
```

---

## 🛠️ Local Development & Quickstart Setup

### Codespaces / Web-Preview Optimization
To run the live workspace engine preview directly inside GitHub Codespaces or standard web browsers via Vite development nodes:
```bash
npm install
npm run dev
```

### Native Desktop Application Execution (Tauri Pipeline)
To compile and instantiate the cross-platform production-ready window binary wrapper leveraging deep native integration:
```bash
# Installs frontend interface frameworks
npm install

# Compiles Rust mathematical targets and launches native platform window
npm run tauri dev
```

---

## 🌌 Author & Developer

Developed with passion by **Yağız Yağlı**. 
Focusing on the alignment of cutting-edge software paradigms with deep physical hardware systems execution mechanics.

Yağız Yağlı: [@yagizyagli](https://github.com/yagizyagli)
Live Demo: [q-code](https://yagizyagli.github.io/q-code/)

---

## ⭐ Support & Contribute
If you find this project's structural architecture or multi-dialect compiler implementation valuable, please consider **dropping a Star** on the repository to spread visibility throughout the open-source quantum movement!
