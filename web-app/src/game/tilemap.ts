import { Container, Graphics } from 'pixi.js';
import { TILE_SIZE, COLORS } from './constants';

export class TilemapRenderer {
  private container = new Container();
  private tileCache: Graphics[][] = [];
  private prevMap: { kind: string; building: string | null }[][] | null = null;
  get display() { return this.container; }

  render(map: { kind: string; building: string | null }[][]): void {
    for (let y = 0; y < map.length; y++) {
      this.tileCache[y] ??= [];
      for (let x = 0; x < map[y].length; x++) {
        const tile = map[y][x];
        // Skip tiles that haven't changed since last render
        const prevTile = this.prevMap?.[y]?.[x];
        if (prevTile && prevTile.kind === tile.kind && prevTile.building === tile.building) {
          continue;
        }

        let gfx = this.tileCache[y][x];
        if (!gfx) {
          gfx = new Graphics();
          gfx.rect(x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE);
          gfx.stroke({ width: 0.5, color: 0x000000, alpha: 0.15 });
          this.tileCache[y][x] = gfx;
          this.container.addChild(gfx);
        }
        const color = COLORS[tile.kind as keyof typeof COLORS] ?? COLORS.grass;
        gfx.clear();
        gfx.rect(x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE);
        gfx.fill(color);
        gfx.rect(x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE);
        gfx.stroke({ width: 0.5, color: 0x000000, alpha: 0.15 });
      }
    }
    this.prevMap = map;
  }

  destroy(): void { this.container.destroy({ children: true }); }
}