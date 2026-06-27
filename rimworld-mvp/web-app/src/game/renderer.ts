import { Application, Container, Graphics } from 'pixi.js';
import { TilemapRenderer } from './tilemap';
import { UnitRenderer, UnitSpriteData } from './sprites';
import { CANVAS_WIDTH, CANVAS_HEIGHT } from './constants';

export interface GameStateData {
  units: UnitSpriteData[];
  map: { kind: string; building: string | null }[][];
}

export class GameRenderer {
  private app!: Application;
  private tilemap = new TilemapRenderer();
  private units = new UnitRenderer();
  private buildings = new Container();

  async init(canvas: HTMLCanvasElement): Promise<void> {
    this.app = new Application();
    await this.app.init({ canvas, width: CANVAS_WIDTH, height: CANVAS_HEIGHT, background: 0x1a1a2e });
    this.app.stage.addChild(this.tilemap.display, this.buildings, this.units.display);
  }

  private renderBuildings(map: { kind: string; building: string | null }[][]): void {
    this.buildings.removeChildren();
    for (let y = 0; y < map.length; y++) {
      for (let x = 0; x < map[y].length; x++) {
        const b = map[y][x].building;
        if (!b) continue;
        const gfx = new Graphics(), px = x * 16, py = y * 16;
        if (b === 'berry_bush') { gfx.circle(px + 8, py + 8, 4).fill(0xcc3333); gfx.circle(px + 6, py + 6, 2).fill(0x33cc33); }
        else if (b === 'bed') { gfx.rect(px + 2, py + 6, 12, 6).fill(0x8b4513); gfx.rect(px + 1, py + 4, 14, 3).fill(0x654321); }
        this.buildings.addChild(gfx);
      }
    }
  }

  render(state: GameStateData): void {
    this.tilemap.render(state.map);
    this.renderBuildings(state.map);
    this.units.render(state.units);
  }

  getCanvasCoords(globalX: number, globalY: number): { x: number; y: number } | null {
    const r = this.app.canvas.getBoundingClientRect();
    const cx = globalX - r.left, cy = globalY - r.top;
    if (cx < 0 || cx > CANVAS_WIDTH || cy < 0 || cy > CANVAS_HEIGHT) return null;
    return { x: Math.floor(cx / 16), y: Math.floor(cy / 16) };
  }

  destroy(): void { this.tilemap.destroy(); this.units.destroy(); this.buildings.destroy({ children: true }); this.app.destroy(); }
}