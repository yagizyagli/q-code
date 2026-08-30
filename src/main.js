// Q-Code IDE | Interactive Frontend Engine Core (Safe CDN Fallback Build)

// Multi-dialect pre-defined code templates for the workspace file explorer simulation
const INTERACTIVE_FILES = {
    'algorithm.qasm': {
        code: `OPENQASM 3.0;\ninclude "stdgates.inc";\n\nqubit q;\nbit c;\n\nh q;\nx q;\n\nc = measure q;`,
        lang: 'OpenQASM 3.0',
        dialect: 'OpenQASM'
    },
    'operation.qs': {
        code: `namespace Quantum.Core {\n    open Microsoft.Quantum.Intrinsic;\n\n    operation RunCircuit() : Unit {\n        use q = Qubit();\n        H(q);\n        X(q);\n    }\n}`,
        lang: 'Q# (Quantum Sharp)',
        dialect: 'QSharp'
    }
};

// 3D Bloch Sphere Graphical Renderer Subsystem
class QuantumBlochSphere {
    constructor(container) {
        const THREE = window.THREE;
        this.scene = new THREE.Scene();
        this.camera = new THREE.PerspectiveCamera(45, container.clientWidth / container.clientHeight, 0.1, 100);
        this.camera.position.set(0, 0, 5);

        this.renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
        this.renderer.setSize(container.clientWidth, container.clientHeight);
        container.appendChild(this.renderer.domElement);

        const geometry = new THREE.SphereGeometry(1.4, 32, 32);
        const material = new THREE.MeshBasicMaterial({ color: 0x4a5568, wireframe: true, transparent: true, opacity: 0.15 });
        this.sphere = new THREE.Mesh(geometry, material);
        this.scene.add(this.sphere);

        const axesHelper = new THREE.AxesHelper(1.8);
        this.scene.add(axesHelper);

        const dir = new THREE.Vector3(0, 1, 0);
        const origin = new THREE.Vector3(0, 0, 0);
        this.vectorArrow = new THREE.ArrowHelper(dir, origin, 1.4, 0x00f3ff, 0.2, 0.1);
        this.scene.add(this.vectorArrow);
    }

    updateState(theta, phi) {
        const THREE = window.THREE;
        const x = Math.sin(theta) * Math.cos(phi);
        const z = Math.sin(theta) * Math.sin(phi);
        const y = Math.cos(theta);
        const newDirection = new THREE.Vector3(x, y, z).normalize();
        this.vectorArrow.setDirection(newDirection);
    }

    animate = () => {
        requestAnimationFrame(this.animate);
        this.sphere.rotation.y += 0.002;
        this.renderer.render(this.scene, this.camera);
    }
}

// 2D Quantum Schematic Wire Canvas Renderer Subsystem
class QuantumCircuitRenderer {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        this.ctx = this.canvas.getContext('2d');
        this.resizeCanvas();
    }

    resizeCanvas() {
        const rect = this.canvas.parentElement.getBoundingClientRect();
        this.canvas.width = rect.width;
        this.canvas.height = 110;
    }

    renderCircuit(gatesCount, dialect) {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        this.ctx.strokeStyle = '#23283b';
        this.ctx.lineWidth = 2;
        
        this.ctx.beginPath(); this.ctx.moveTo(20, 40); this.ctx.lineTo(350, 40); this.ctx.stroke();
        this.ctx.beginPath(); this.ctx.moveTo(20, 80); this.ctx.lineTo(350, 80); this.ctx.stroke();

        this.ctx.fillStyle = '#718096';
        this.ctx.font = '12px monospace';
        this.ctx.fillText('q0 ──', 10, 44);
        this.ctx.fillText('q1 ──', 10, 84);

        if (gatesCount > 0) {
            this.ctx.fillStyle = '#00f3ff';
            this.ctx.fillRect(80, 25, 30, 30);
            this.ctx.fillStyle = '#0c0d12';
            this.ctx.font = 'bold 14px sans-serif';
            this.ctx.fillText('H', 90, 46);

            this.ctx.fillStyle = '#a0aec0';
            this.ctx.fillRect(140, 25, 30, 30);
            this.ctx.fillStyle = '#0c0d12';
            this.ctx.font = 'bold 14px sans-serif';
            this.ctx.fillText('X', 150, 46);

            this.ctx.fillStyle = '#1a1d29';
            this.ctx.fillRect(260, 5, 80, 20);
            this.ctx.fillStyle = '#00f3ff';
            this.ctx.font = '10px monospace';
            this.ctx.fillText(dialect, 270, 18);
        }
    }
}

// -------------------------------------------------------------
// SECURE CORE INITIALIZATION LOOP (Waits safely for Monaco Engine)
// -------------------------------------------------------------
const initializeIDE = () => {
    const monaco = window.monaco;
    const editorContainer = document.getElementById('monaco-root');
    
    const editor = monaco.editor.create(editorContainer, {
        value: INTERACTIVE_FILES['algorithm.qasm'].code,
        language: 'javascript', 
        theme: 'vs-dark',
        automaticLayout: true,
        fontSize: 14,
        fontFamily: 'Fira Code, monospace'
    });

    const blochContainer = document.getElementById('bloch-output');
    const blochSphere = new QuantumBlochSphere(blochContainer);
    blochSphere.animate();

    const circuitRenderer = new QuantumCircuitRenderer('circuit-canvas');
    circuitRenderer.renderCircuit(2, "OpenQASM");

    const printLog = (msg, type = 'info') => {
        const body = document.querySelector('.terminal-body');
        if (body) {
            const line = document.createElement('div');
            line.className = `log-line ${type}`;
            line.textContent = `[${new Date().toLocaleTimeString()}] ${msg}`;
            body.appendChild(line);
            body.scrollTop = body.scrollHeight;
        }
    };

    const uiElements = {
        fileQasm: document.getElementById('file-qasm'),
        fileQsharp: document.getElementById('file-qsharp'),
        tabQasm: document.getElementById('tab-qasm'),
        tabQsharp: document.getElementById('tab-qsharp'),
        uiDialect: document.getElementById('ui-dialect'),
        uiStatusLang: document.getElementById('ui-status-lang')
    };

    const handleWorkspaceRouting = (targetFile) => {
        const fileData = INTERACTIVE_FILES[targetFile];
        editor.setValue(fileData.code);
        uiElements.uiDialect.textContent = fileData.dialect;
        uiElements.uiStatusLang.textContent = `Language: ${fileData.lang}`;
        printLog(`Workspace controller loaded active memory stream: "${targetFile}"`, 'info');

        if (targetFile === 'algorithm.qasm') {
            uiElements.fileQasm.classList.add('active'); uiElements.tabQasm.classList.add('active');
            uiElements.fileQsharp.classList.remove('active'); uiElements.tabQsharp.classList.remove('active');
        } else {
            uiElements.fileQsharp.classList.add('active'); uiElements.tabQsharp.classList.add('active');
            uiElements.fileQasm.classList.remove('active'); uiElements.tabQasm.classList.remove('active');
        }
    };

    uiElements.fileQasm.addEventListener('click', () => handleWorkspaceRouting('algorithm.qasm'));
    uiElements.tabQasm.addEventListener('click', () => handleWorkspaceRouting('algorithm.qasm'));
    uiElements.fileQsharp.addEventListener('click', () => handleWorkspaceRouting('operation.qs'));
    uiElements.tabQsharp.addEventListener('click', () => handleWorkspaceRouting('operation.qs'));

    let timeoutId;
    editor.onDidChangeModelContent(() => {
        clearTimeout(timeoutId);
        timeoutId = setTimeout(async () => {
            const currentCode = editor.getValue();
            let parsedData;

            try {
                if (window.__TAURI_INTERNALS__) {
                    const { invoke } = await import('@tauri-apps/api/core');
                    const response = await invoke('analyze_quantum_code', { code: currentCode });
                    parsedData = JSON.parse(response);
                } else {
                    const isQSharp = currentCode.includes("namespace") || currentCode.includes("operation");
                    const containsGates = currentCode.toLowerCase().includes('h ') || currentCode.toLowerCase().includes('h(') ||
                                         currentCode.toLowerCase().includes('x ') || currentCode.toLowerCase().includes('x(');

                    parsedData = { status: "success", dialect: isQSharp ? "QSharp" : "OpenQASM", nodes_parsed: containsGates ? 2 : 0 };
                    printLog(`Cloud Web Compiler evaluated code buffer variables [Dialect: ${parsedData.dialect}].`, 'success');
                }

                if (parsedData.status === "success") {
                    if (parsedData.nodes_parsed > 0) {
                        blochSphere.updateState(Math.PI / 4, Math.PI / 4);
                        circuitRenderer.renderCircuit(parsedData.nodes_parsed, parsedData.dialect);
                    } else {
                        blochSphere.updateState(0, 0);
                        circuitRenderer.renderCircuit(0, parsedData.dialect);
                        printLog(`Zero quantum operators mapped. Resetting telemetry matrix channels.`, 'warning');
                    }
                }
            } catch (error) {
                console.error(error);
                printLog(`Pipeline execution framework fault trapped.`, 'warning');
            }
        }, 500);
    });
};

// Asynchronous polling mechanism to prevent lifecycle execution racing faults
const checkMonaco = () => {
    if (window.monaco && window.THREE) {
        initializeIDE();
    } else {
        setTimeout(checkMonaco, 50); // Checks every 50ms until libraries exist
    }
};

document.addEventListener('DOMContentLoaded', checkMonaco);
