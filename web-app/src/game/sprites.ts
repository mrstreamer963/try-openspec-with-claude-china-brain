import { Container, Graphics, Text } from 'pixi.js';
import { TILE_SIZE, COLORS } from './constants';

export interface UnitSpriteData {
  id: number; name: string; x: number; y: number;
  food: number; energy: number; state: string; debuffs: string[];
}

export class UnitRenderer {
  private container = new Container();
  private cache = new Map<number, { body: Graphics; barBg: Graphics; barFood: Graphics; barEnergy: Graphics; debuffText?: Text }>();
  get display() { return this.container; }

  render(units: UnitSpriteData[]): void {
    const active = new Set(units.map(u => u.id));
    for (const [id, g] of this.cache) {
      if (!active.has(id)) { this.container.removeChild(g.body, g.barBg, g.barFood, g.barEnergy); this.cache.delete(id); }
    }
    for (const unit of units) {
      let g = this.cache.get(unit.id);
      const px = unit.x * TILE_SIZE;
      const py = unit.y * TILE_SIZE;
      if (!g) {
        const body = new Graphics(), barBg = new Graphics(), barFood = new Graphics(), barEnergy = new Graphics();
        this.container.addChild(barBg, barFood, barEnergy, body);
        g = { body, barBg, barFood, barEnergy };
        this.cache.set(unit.id, g);
      }
      const ci = unit.id % COLORS.unit.length;
      g.body.clear();
      if (unit.state === 'Sleeping') {
        g.body.rect(px + 2, py + TILE_SIZE / 2 - 2, TILE_SIZE - 4, 4);
      } else {
        g.body.circle(px + TILE_SIZE / 2, py + TILE_SIZE / 2, 5);
      }
      g.body.fill(COLORS.unit[ci]);
      g.barBg.clear().rect(px, py - 4, TILE_SIZE, 3).fill(COLORS.bar_bg);
      g.barFood.clear().rect(px, py - 4, TILE_SIZE * (unit.food / 100), 3).fill(COLORS.bar_food);
      g.barEnergy.clear().rect(px, py - 8, TILE_SIZE * (unit.energy / 100), 1).fill(COLORS.bar_energy);
    }
  }

  destroy(): void { this.container.destroy({ children: true }); this.cache.clear(); }
}