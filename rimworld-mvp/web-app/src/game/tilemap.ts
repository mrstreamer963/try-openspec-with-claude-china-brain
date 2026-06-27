import { Container, Graphics } from 'pixi.js';
import { TILE_SIZE, COLORS } from './constants';

export class TilemapRenderer {
  private container = new Container();
  get display() { return this.container; }

  render(map: { kind: string; building: string | null }[][]): void {
    this.container.removeChildren();
    for (let y = 0; y < map.length; y++) {
      for (let x = 0; x < map[y].length; x++) {
        const tile = map[y][x];
        const gfx = new Graphics();
        const color = COLORS[tile.kind as keyof typeof COLORS] ?? COLORS.grass;
        gfx.rect(x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE);
        gfx.fill(color);
        gfx.rect(x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE);
        gfx.stroke({ width: 0.5, color: 0x000000, alpha: 0.15 });
        this.container.addChild(gfx);
      }
    }
  }

  destroy(): void { this.container.destroy({ children: true }); }
}