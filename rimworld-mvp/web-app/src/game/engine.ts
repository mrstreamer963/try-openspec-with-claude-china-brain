import { GameRenderer } from './renderer';
import { useGameStore } from '../store/gameStore';

export class GameEngine {
  private worker: Worker;
  private renderer: GameRenderer;
  private lastTime = 0;
  private rafHandle = 0;
  private store = useGameStore();
  private canvas: HTMLCanvasElement;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.worker = new Worker(new URL('../worker/game-bridge.ts', import.meta.url), { type: 'module' });
    this.renderer = new GameRenderer();

    this.worker.onmessage = (e) => {
      const { type, payload } = e.data;
      if (type === 'initialized') { this.store.setInitialized(); this.lastTime = performance.now(); requestAnimationFrame(this.loop); }
      if (type === 'state_update') { this.store.setState(payload.state); this.renderer.render(payload.state); }
    };

    this.renderer.init(canvas);
    this.worker.postMessage({ type: 'init' });
    canvas.addEventListener('pointerdown', this.onPointerDown.bind(this));
    canvas.addEventListener('contextmenu', (e) => e.preventDefault());
  }

  private loop = (time: number) => {
    const rawDt = (time - this.lastTime) / 1000;
    this.lastTime = time;
    if (!this.store.isPaused) {
      this.worker.postMessage({ type: 'update', payload: { dt: Math.min(rawDt, 0.1) * this.store.gameSpeed } });
    }
    this.rafHandle = requestAnimationFrame(this.loop);
  };

  private onPointerDown(event: PointerEvent) {
    const coords = this.renderer.getCanvasCoords(event.clientX, event.clientY);
    if (!coords || !this.store.gameState) return;
    if (event.button === 2) { this.store.selectUnit(null); return; }  // right-click deselect

    if (this.store.buildMode) {
      this.worker.postMessage({ type: 'command', payload: { command: { PlaceBuilding: { x: coords.x, y: coords.y, kind: this.store.buildMode } } } });
      this.store.setBuildMode(null);
      return;
    }

    const clickedUnit = this.store.gameState.units.find(u => Math.floor(u.x) === coords.x && Math.floor(u.y) === coords.y);
    if (clickedUnit) { this.store.selectUnit(clickedUnit.id); return; }

    if (this.store.selectedUnitId !== null) {
      this.worker.postMessage({ type: 'command', payload: { command: { MoveUnit: { unit_id: this.store.selectedUnitId, x: coords.x + 0.5, y: coords.y + 0.5 } } } });
    }
  }

  destroy() { cancelAnimationFrame(this.rafHandle); this.worker.terminate(); this.renderer.destroy(); }
}