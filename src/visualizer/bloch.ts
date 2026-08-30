import * as THREE from 'three';

export class QuantumBlochSphere {
    private scene: THREE.Scene;
    private camera: THREE.PerspectiveCamera;
    private renderer: THREE.WebGLRenderer;
    private sphere: THREE.Mesh;
    private vectorArrow: THREE.ArrowHelper;

    constructor(container: HTMLElement) {
        this.scene = new THREE.Scene();
        this.camera = new THREE.PerspectiveCamera(45, container.clientWidth / container.clientHeight, 0.1, 100);
        this.camera.position.set(0, 0, 5);

        this.renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
        this.renderer.setSize(container.clientWidth, container.clientHeight);
        container.appendChild(this.renderer.domElement);

        // Render Geometry: Transparent Bloch Sphere Wireframe
        const geometry = new THREE.SphereGeometry(1.5, 32, 32);
        const material = new THREE.MeshBasicMaterial({
            color: 0x4a5568,
            wireframe: true,
            transparent: true,
            opacity: 0.15
        });
        this.sphere = new THREE.Mesh(geometry, material);
        this.scene.add(this.sphere);

        // Render Coordinate Spaces: X, Y, Z Quantum Axes Setup
        const axesHelper = new THREE.AxesHelper(2);
        this.scene.add(axesHelper);

        // Ground State Vector Initialization (|0> state pointing upwards)
        const dir = new THREE.Vector3(0, 1, 0);
        const origin = new THREE.Vector3(0, 0, 0);
        this.vectorArrow = new THREE.ArrowHelper(dir, origin, 1.5, 0x00f3ff, 0.2, 0.1);
        this.scene.add(this.vectorArrow);

        // Ambient Illumination Setup
        const light = new THREE.AmbientLight(0xffffff, 1);
        this.scene.add(light);
    }

    /**
     * Updates the physical state vector orientation using Theta and Phi probability amplitudes
     */
    public updateState(theta: number, phi: number) {
        const x = Math.sin(theta) * Math.cos(phi);
        const z = Math.sin(theta) * Math.sin(phi);
        const y = Math.cos(theta); // Z-axis represents the vertical computational basis in quantum mechanics

        const newDirection = new THREE.Vector3(x, y, z).normalize();
        this.vectorArrow.setDirection(newDirection);
    }

    /**
     * Primary animation loop executed via Hardware-Accelerated RequestAnimationFrame
     */
    public animate = () => {
        requestAnimationFrame(this.animate);
        this.sphere.rotation.y += 0.002; // Idle rotation effect for immersive UX depth
        this.renderer.render(this.scene, this.camera);
    }
}
