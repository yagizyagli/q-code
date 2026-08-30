export class QuantumCircuitRenderer {
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;

    constructor(canvasId: string) {
        this.canvas = document.getElementById(canvasId) as HTMLCanvasElement;
        this.ctx = this.canvas.getContext('2d')!;
        this.resizeCanvas();
    }

    private resizeCanvas() {
        // Handle pixel density for crisp retina displays in engineering software
        const rect = this.canvas.parentElement!.getBoundingClientRect();
        this.canvas.width = rect.width * window.devicePixelRatio;
        this.canvas.height = 150 * window.devicePixelRatio; // Fixed heights for quantum timeline registers
        this.ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    }

    /**
     * Renders standard quantum register timelines (qubit wires) and visualizes gate triggers
     */
    public renderCircuit(gatesCount: number, dialect: string) {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        
        // Draw 2 Parallel Qubit Timeline Wires (q[0] and q[1])
        this.ctx.strokeStyle = '#23283b';
        this.ctx.lineWidth = 2;
        
        // Wire 0 (Qubit 0)
        this.ctx.beginPath();
        this.ctx.moveTo(20, 50);
        this.ctx.lineTo(400, 50);
        this.ctx.stroke();

        // Wire 1 (Qubit 1)
        this.ctx.beginPath();
        this.ctx.moveTo(20, 100);
        this.ctx.lineTo(400, 100);
        this.ctx.stroke();

        // Annotate Register Labels
        this.ctx.fillStyle = '#718096';
        this.ctx.font = '12px monospace';
        this.ctx.fillText('q[0] ───', 10, 54);
        this.ctx.fillText('q[1] ───', 10, 104);

        // If the compiler detected quantum instructions, overlay the graphical schematic blocks
        if (gatesCount > 0) {
            // Draw Hadamard Gate Matrix Block on Qubit Wire 0
            this.ctx.fillStyle = '#00f3ff';
            this.ctx.fillRect(80, 35, 30, 30);
            
            this.ctx.fillStyle = '#0c0d12';
            this.ctx.font = 'bold 14px sans-serif';
            this.ctx.fillText('H', 90, 56);

            // Draw dialect identifier telemetry badge
            this.ctx.fillStyle = '#1a1d29';
            this.ctx.fillRect(300, 5, 80, 20);
            this.ctx.fillStyle = '#00f3ff';
            this.ctx.font = '10px monospace';
            this.ctx.fillText(dialect, 310, 18);
        }
    }
}
