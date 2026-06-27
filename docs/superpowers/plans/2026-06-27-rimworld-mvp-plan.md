# RimWorld MVP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) for syntax tracking.

**Goal:** Build a browser-based colony sim MVP with Rust/WASM game core (bevy_ecs) running in a Web Worker, rendered via PixiJS/Vue 3 on the main thread.

**Architecture:** Monorepo with Cargo workspace (wasm-core crate) + Vite/Vue web-app. WASM core runs in Web Worker, communicates via postMessage with JSON state + events. PixiJS renders tilemap + sprites on canvas, Vue overlays UI panels.

**Tech Stack:** Rust (bevy_ecs 0.15, wasm-bindgen, serde_json), Vite 6, Vue 3 (Composition API), PixiJS v8, TypeScript, Pinia

## Global Constraints

- Map size constant `MAP_SIZE` as `u16` (value 30 for MVP)
- Tile size 16×16 pixels on canvas (480×480 total)
- All game logic in Rust wasm-core; rendering + UI in JS
- Web Worker communication via structured clone (JSON-serializable data)
- 3 units start in a 5-tile radius from map center (15,15)
- No unit death; no combat; no save/load

---

### Task 1: Project Scaffolding

**Files:**
- Create: `rimworld-mvp/Cargo.toml`
- Create: `rimworld-mvp/wasm-core/Cargo.toml`
- Create: `rimworld-mvp/wasm-core/src/lib.rs`
- Create: `rimworld-mvp/web-app/package.json`
- Create: `rimworld-mvp/web-app/vite.config.ts`
- Create: `rimworld-mvp/web-app/tsconfig.json`
- Create: `rimworld-mvp/web-app/index.html`
- Create: `rimworld-mvp/web-app/src/main.ts`
- Create: `rimworld-mvp/web-app/src/App.vue`

**Interfaces:**
- Consumes: nothing
- Produces: compilable Rust crate + Vite dev server

**Note on project root:** All paths relative to `/Users/mr.streamer/restore/try-openspec-with-claude-china-brain/rimworld-mvp/`.

- [ ] **Step 1: Create Cargo workspace root** (`rimworld-mvp/Cargo.toml`)

```toml
[workspace]
members = ["wasm-core"]
resolver = "2"
```

- [ ] **Step 2: Create wasm-core Cargo.toml**

```toml
[package]
name = "wasm-core"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
bevy_ecs = "0.15"
wasm-bindgen = "0.2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[profile.release]
opt-level = "s"
lto = true
```

- [ ] **Step 3: Create stub lib.rs**

```rust
pub mod components;
pub mod map;
pub mod systems;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() { assert!(true); }
}
```

- [ ] **Step 4: Verify Rust compiles** — run `cargo check`

- [ ] **Step 5: Create web-app/package.json**

```json
{
  "name": "rimworld-mvp-web",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": { "dev": "vite", "build": "vite build", "preview": "vite preview" },
  "dependencies": {
    "vue": "^3.5",
    "pixi.js": "^8.0",
    "pinia": "^2.1"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^5.0",
    "typescript": "^5.5",
    "vite": "^6.0"
  }
}
```

- [ ] **Step 6: Create vite.config.ts**

```typescript
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
export default defineConfig({
  plugins: [vue()],
  worker: { format: 'es' },
})
```

- [ ] **Step 7: Create tsconfig.json**

```json
{
  "compilerOptions": {
    "target": "ESNext",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "strict": true,
    "skipLibCheck": true,
    "paths": { "@/*": ["./src/*"] }
  },
  "include": ["src/**/*.ts", "src/**/*.vue"]
}
```

- [ ] **Step 8: Create index.html**

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>RimWorld MVP</title>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { background: #1a1a2e; color: #eee; font-family: monospace; overflow: hidden; }
    #app { display: flex; flex-direction: column; height: 100vh; }
  </style>
</head>
<body>
  <div id="app"></div>
  <script type="module" src="/src/main.ts"></script>
</body>
</html>
```

- [ ] **Step 9: Create src/main.ts**

```typescript
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
const app = createApp(App)
app.use(createPinia())
app.mount('#app')
```

- [ ] **Step 10: Create src/App.vue (minimal)**

```vue
<template>
  <div id="game-container">
    <canvas id="game-canvas"></canvas>
  </div>
</template>
<script setup lang="ts">
</script>
<style scoped>
#game-container { flex: 1; display: flex; align-items: center; justify-content: center; }
</style>
```

- [ ] **Step 11: Install deps** — run `cd rimworld-mvp/web-app && npm install`

- [ ] **Step 12: Commit**

```bash
git add rimworld-mvp/
git commit -m "feat: scaffold Cargo workspace + Vite/Vue/PixiJS project"
```

---

### Task 2: Rust Core — ECS Components and Map

**Files:**
- Create: `rimworld-mvp/wasm-core/src/components.rs`
- Create: `rimworld-mvp/wasm-core/src/map.rs`

**Interfaces:**
- Consumes: nothing
- Produces: `components::*` (all ECS types, resources), `GameMap::new(u16)`, tile accessors

- [ ] **Step 1: Write components.rs**

```rust
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

pub const MAP_SIZE: u16 = 30;

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Position { pub x: f32, pub y: f32 }

#[derive(Component, Debug)]
pub struct Unit { pub id: u32, pub name: String }

#[derive(Component, Debug)]
pub struct Needs { pub food: f32, pub energy: f32 }

#[derive(Component, Debug)]
pub struct Target(pub Option<(f32, f32)>);

#[derive(Component, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnitState { Idle, Moving, Eating, Sleeping }

#[derive(Component, Debug)]
pub struct CurrentState(pub UnitState);

#[derive(Component, Debug)]
pub struct Speed(pub f32);

#[derive(Resource, Debug)]
pub struct Time { pub delta_seconds: f32 }

#[derive(Resource, Debug)]
pub struct GameRng(pub u32);

#[derive(Component, Debug)]
pub struct Bed;

#[derive(Component, Debug)]
pub struct BerryBush;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TileKind { Water, Sand, Grass }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Building { BerryBush, Bed }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub kind: TileKind,
    pub building: Option<Building>,
}
```

- [ ] **Step 2: Write map.rs**

```rust
use crate::components::*;

pub struct GameMap {
    pub tiles: Vec<Vec<Tile>>,
    pub size: u16,
}

impl GameMap {
    pub fn new(size: u16) -> Self {
        let mut tiles = Vec::with_capacity(size as usize);
        for y in 0..size {
            let mut row = Vec::with_capacity(size as usize);
            for x in 0..size {
                row.push(Tile { kind: Self::generate_tile(x, y, size), building: None });
            }
            tiles.push(row);
        }
        GameMap { tiles, size }
    }

    fn generate_tile(x: u16, y: u16, size: u16) -> TileKind {
        let h = (x as u32).wrapping_mul(0x9e3779b9)
            .wrapping_add(y as u32)
            .wrapping_mul(0x9e3779b9);
        let r = h % 100;
        let center = size as f32 / 2.0;
        let dx = (x as f32 - center).abs();
        let dy = (y as f32 - center).abs();
        if dx < 5.0 && dy < 5.0 { return TileKind::Grass; }
        if x < 2 || y < 2 || x >= size - 2 || y >= size - 2 {
            if r < 60 { return TileKind::Water; }
        }
        match r {
            0..=14 => TileKind::Water,
            15..=39 => TileKind::Sand,
            _ => TileKind::Grass,
        }
    }

    pub fn is_walkable(&self, x: f32, y: f32) -> bool {
        let gx = x.floor() as u16;
        let gy = y.floor() as u16;
        if gx >= self.size || gy >= self.size { return false; }
        self.tiles[gy as usize][gx as usize].kind != TileKind::Water
    }

    pub fn get_tile(&self, x: u16, y: u16) -> Option<&Tile> {
        if x >= self.size || y >= self.size { return None; }
        Some(&self.tiles[y as usize][x as usize])
    }

    pub fn get_tile_mut(&mut self, x: u16, y: u16) -> Option<&mut Tile> {
        if x >= self.size || y >= self.size { return None; }
        Some(&mut self.tiles[y as usize][x as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_size() {
        let map = GameMap::new(30);
        assert_eq!(map.tiles.len(), 30);
        assert_eq!(map.tiles[0].len(), 30);
    }

    #[test]
    fn test_center_walkable() {
        let map = GameMap::new(30);
        assert!(map.is_walkable(15.0, 15.0));
    }

    #[test]
    fn test_water_not_walkable() {
        let map = GameMap::new(30);
        for y in 0..30u16 {
            for x in 0..30u16 {
                let tile = map.get_tile(x, y).unwrap();
                assert_eq!(map.is_walkable(x as f32, y as f32), tile.kind != TileKind::Water);
            }
        }
    }
}
```

- [ ] **Step 3: Run tests** — run `cargo test`, expect all pass
- [ ] **Step 4: Commit**

```bash
git add rimworld-mvp/wasm-core/src/components.rs rimworld-mvp/wasm-core/src/map.rs
git commit -m "feat(w-core): ECS component types and map generation"
```

---

### Task 3: Rust Core — Game Systems

**Files:**
- Create: `rimworld-mvp/wasm-core/src/systems/mod.rs`
- Create: `rimworld-mvp/wasm-core/src/systems/needs.rs`
- Create: `rimworld-mvp/wasm-core/src/systems/movement.rs`
- Create: `rimworld-mvp/wasm-core/src/systems/idle.rs`
- Create: `rimworld-mvp/wasm-core/src/systems/auto_need.rs`
- Create: `rimworld-mvp/wasm-core/src/systems/player_commands.rs`

**Interfaces:**
- Consumes: `components::*`, `map::GameMap`
- Produces: system functions, `PlayerCommand` + `BuildingKind` enums, `CommandQueue` resource

- [ ] **Step 1: Create systems/mod.rs**

```rust
pub mod needs;
pub mod movement;
pub mod idle;
pub mod auto_need;
pub mod player_commands;
```

- [ ] **Step 2: Write systems/needs.rs**

```rust
use bevy_ecs::prelude::*;
use crate::components::*;

pub fn need_decay_system(
    mut query: Query<(&mut Needs, &CurrentState)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds;
    for (mut needs, state) in query.iter_mut() {
        match state.0 {
            UnitState::Eating => {
                needs.food = (needs.food + 20.0 * dt).min(100.0);
                needs.energy = (needs.energy - 1.5 * dt).max(0.0);
            }
            UnitState::Sleeping => {
                needs.energy = (needs.energy + 15.0 * dt).min(100.0);
                needs.food = (needs.food - 2.0 * dt).max(0.0);
            }
            _ => {
                needs.food = (needs.food - 2.0 * dt).max(0.0);
                needs.energy = (needs.energy - 1.5 * dt).max(0.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::world::World;

    #[test]
    fn test_normal_decay() {
        let mut world = World::new();
        world.insert_resource(Time { delta_seconds: 1.0 });
        world.spawn((Needs { food: 100.0, energy: 100.0 }, CurrentState(UnitState::Idle)));
        let mut system = IntoSystem::into_system(need_decay_system);
        system.initialize(&mut world);
        system.run((), &mut world);
        let needs = world.query::<&Needs>().iter(&world).next().unwrap();
        assert!(needs.food < 100.0);
        assert!(needs.energy < 100.0);
    }

    #[test]
    fn test_eating_restores_food() {
        let mut world = World::new();
        world.insert_resource(Time { delta_seconds: 1.0 });
        world.spawn((Needs { food: 50.0, energy: 100.0 }, CurrentState(UnitState::Eating)));
        let mut system = IntoSystem::into_system(need_decay_system);
        system.initialize(&mut world);
        system.run((), &mut world);
        let needs = world.query::<&Needs>().iter(&world).next().unwrap();
        assert!(needs.food > 50.0);
    }
}
```

- [ ] **Step 3: Write systems/movement.rs**

```rust
use bevy_ecs::prelude::*;
use crate::components::*;
use crate::map::GameMap;

const MOVE_SPEED: f32 = 3.0;
const ARRIVAL_DIST: f32 = 0.2;

/// Moves units toward their target. On arrival: if a building at target tile,
/// switch to Eating/Sleeping. Otherwise set Idle.
pub fn movement_system(
    mut unit_query: Query<(&mut Position, &mut Target, &mut CurrentState, &Speed)>,
    map: Res<GameMap>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds;
    for (mut pos, mut target, mut state, speed) in unit_query.iter_mut() {
        let target_pos = match &target.0 {
            Some(t) => *t,
            None => continue,
        };

        let dx = target_pos.0 - pos.x;
        let dy = target_pos.1 - pos.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < ARRIVAL_DIST {
            pos.x = target_pos.0;
            pos.y = target_pos.1;
            target.0 = None;

            // Check if target tile has a building
            let gx = pos.x.floor() as u16;
            let gy = pos.y.floor() as u16;
            let building = map.get_tile(gx, gy).and_then(|t| t.building.as_ref());
            match building {
                Some(Building::BerryBush) => state.0 = UnitState::Eating,
                Some(Building::Bed) => state.0 = UnitState::Sleeping,
                None => state.0 = UnitState::Idle,
            }
        } else {
            let step = MOVE_SPEED * speed.0 * dt;
            let travel = step.min(dist);
            pos.x += (dx / dist) * travel;
            pos.y += (dy / dist) * travel;
            state.0 = UnitState::Moving;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::world::World;

    #[test]
    fn test_unit_moves_toward_target() {
        let mut world = World::new();
        world.insert_resource(Time { delta_seconds: 0.1 });
        world.insert_resource(GameMap::new(30));
        world.spawn((
            Position { x: 10.0, y: 10.0 },
            Target(Some((20.0, 10.0))),
            CurrentState(UnitState::Idle),
            Speed(1.0),
        ));
        let mut system = IntoSystem::into_system(movement_system);
        system.initialize(&mut world);
        system.run((), &mut world);
        let pos = world.query::<&Position>().iter(&world).next().unwrap();
        assert!(pos.x > 10.0);
    }

    #[test]
    fn test_unit_arrives_at_target() {
        let mut world = World::new();
        world.insert_resource(Time { delta_seconds: 10.0 });
        world.insert_resource(GameMap::new(30));
        world.spawn((
            Position { x: 10.0, y: 10.0 },
            Target(Some((10.5, 10.0))),
            CurrentState(UnitState::Idle),
            Speed(1.0),
        ));
        let mut system = IntoSystem::into_system(movement_system);
        system.initialize(&mut world);
        system.run((), &mut world);
        let target = world.query::<&Target>().iter(&world).next().unwrap();
        assert_eq!(target.0, None);
    }
}
```

- [ ] **Step 4: Write systems/idle.rs**

```rust
use bevy_ecs::prelude::*;
use crate::components::*;
use crate::map::GameMap;

fn lcg(seed: &mut u32) -> u32 {
    *seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
    *seed % 2147483648
}

pub fn idle_behavior_system(
    mut unit_query: Query<(&mut Target, &CurrentState, &Needs)>,
    map: Res<GameMap>,
    mut seed: ResMut<GameRng>,
) {
    for (mut target, state, needs) in unit_query.iter_mut() {
        if state.0 != UnitState::Idle { continue; }
        if target.0.is_some() { continue; }
        if needs.food < 25.0 || needs.energy < 25.0 { continue; }
        loop {
            let x = (lcg(&mut seed.0) % map.size as u32) as f32 + 0.5;
            let y = (lcg(&mut seed.0) % map.size as u32) as f32 + 0.5;
            if map.is_walkable(x, y) {
                target.0 = Some((x, y));
                break;
            }
        }
    }
}
```

- [ ] **Step 5: Write systems/auto_need.rs**

```rust
use bevy_ecs::prelude::*;
use crate::components::*;

fn dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

pub fn auto_need_system(
    mut unit_query: Query<(&Position, &mut Target, &mut CurrentState, &Needs)>,
    bush_query: Query<&Position, (With<BerryBush>, Without<Bed>)>,
    bed_query: Query<&Position, (With<Bed>, Without<BerryBush>)>,
) {
    for (pos, mut target, mut state, needs) in unit_query.iter_mut() {
        if state.0 != UnitState::Idle && state.0 != UnitState::Moving { continue; }
        if target.0.is_some() { continue; }

        if needs.food < 25.0 {
            let nearest = bush_query.iter()
                .map(|bp| (bp, dist(pos.x, pos.y, bp.x, bp.y)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            if let Some((bush_pos, _)) = nearest {
                target.0 = Some((bush_pos.x, bush_pos.y));
                state.0 = UnitState::Moving;
                continue;
            }
        }

        if needs.energy < 25.0 {
            let nearest = bed_query.iter()
                .map(|bp| (bp, dist(pos.x, pos.y, bp.x, bp.y)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            if let Some((bed_pos, _)) = nearest {
                target.0 = Some((bed_pos.x, bed_pos.y));
                state.0 = UnitState::Moving;
            }
        }
    }
}
```

- [ ] **Step 6: Write systems/player_commands.rs**

```rust
use bevy_ecs::prelude::*;
use serde::Deserialize;
use crate::components::*;
use crate::map::GameMap;

#[derive(Debug, Deserialize)]
pub enum PlayerCommand {
    MoveUnit { unit_id: u32, x: f32, y: f32 },
    PlaceBuilding { x: u16, y: u16, kind: BuildingKind },
}

#[derive(Debug, Deserialize)]
pub enum BuildingKind {
    #[serde(rename = "berry_bush")] BerryBush,
    #[serde(rename = "bed")] Bed,
}

#[derive(Resource, Default, Debug)]
pub struct CommandQueue {
    pub commands: Vec<PlayerCommand>,
}

pub fn player_command_system(
    mut commands: ResMut<CommandQueue>,
    mut unit_query: Query<(&Unit, &mut Target, &mut CurrentState)>,
    mut map: ResMut<GameMap>,
) {
    let mut cmds = std::mem::take(&mut commands.commands);
    for cmd in cmds.drain(..) {
        match cmd {
            PlayerCommand::MoveUnit { unit_id, x, y } => {
                for (unit, mut target, mut state) in unit_query.iter_mut() {
                    if unit.id == unit_id {
                        target.0 = Some((x, y));
                        state.0 = UnitState::Moving;
                        break;
                    }
                }
            }
            PlayerCommand::PlaceBuilding { x, y, kind } => {
                if let Some(tile) = map.get_tile_mut(x, y) {
                    if tile.kind == TileKind::Water { continue; }
                    if tile.building.is_some() { continue; }
                    tile.building = Some(match kind {
                        BuildingKind::BerryBush => Building::BerryBush,
                        BuildingKind::Bed => Building::Bed,
                    });
                }
            }
        }
    }
}
```

- [ ] **Step 7: Run all tests** — `cargo test`

- [ ] **Step 8: Commit**

```bash
git add rimworld-mvp/wasm-core/src/systems/
git commit -m "feat(w-core): game systems — needs, movement, idle AI, auto-need, player commands"
```

---

### Task 4: Rust Core — Events + WASM Entry Point

**Files:**
- Create: `rimworld-mvp/wasm-core/src/events.rs`
- Create: `rimworld-mvp/wasm-core/src/game_state.rs`
- Create: `rimworld-mvp/wasm-core/src/wasm_entry.rs`
- Modify: `rimworld-mvp/wasm-core/src/lib.rs`

**Interfaces:**
- Consumes: all systems, components, map
- Produces: exported `init()`, `update(dt) -> String`, `send_command(json)` wasm functions

- [ ] **Step 1: Create events.rs**

```rust
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GameEvent {
    pub kind: String,
    pub unit_id: Option<u32>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub food: Option<f32>,
    pub energy: Option<f32>,
    pub state: Option<String>,
    pub debuffs: Option<Vec<String>>,
}

impl GameEvent {
    pub fn unit_moved(id: u32, x: f32, y: f32) -> Self {
        Self { kind: "unit_moved".into(), unit_id: Some(id), x: Some(x), y: Some(y), food: None, energy: None, state: None, debuffs: None }
    }
    pub fn needs_changed(id: u32, food: f32, energy: f32) -> Self {
        Self { kind: "needs_changed".into(), unit_id: Some(id), x: None, y: None, food: Some(food), energy: Some(energy), state: None, debuffs: None }
    }
    pub fn state_changed(id: u32, state: String) -> Self {
        Self { kind: "state_changed".into(), unit_id: Some(id), x: None, y: None, food: None, energy: None, state: Some(state), debuffs: None }
    }
    pub fn debuff_changed(id: u32, debuffs: Vec<String>) -> Self {
        Self { kind: "debuff_changed".into(), unit_id: Some(id), x: None, y: None, food: None, energy: None, state: None, debuffs: Some(debuffs) }
    }
    pub fn building_placed(x: f32, y: f32, building: String) -> Self {
        Self { kind: "building_placed".into(), unit_id: None, x: Some(x), y: Some(y), food: None, energy: None, state: None, debuffs: None }
    }
}

#[derive(Resource, Default, Debug)]
pub struct EventLog {
    pub events: Vec<GameEvent>,
}

impl EventLog {
    pub fn push(&mut self, event: GameEvent) { self.events.push(event); }
    pub fn drain_all(&mut self) -> Vec<GameEvent> { self.events.drain(..).collect() }
}
```

- [ ] **Step 2: Create game_state.rs**

```rust
use bevy_ecs::prelude::*;
use serde::Serialize;
use crate::components::*;
use crate::map::GameMap;

#[derive(Serialize)]
pub struct UnitSnapshot {
    pub id: u32, pub name: String, pub x: f32, pub y: f32,
    pub food: f32, pub energy: f32, pub state: String, pub debuffs: Vec<String>,
}

#[derive(Serialize)]
pub struct MapTileSnapshot {
    pub kind: String, pub building: Option<String>,
}

#[derive(Serialize)]
pub struct GameStateSnapshot {
    pub units: Vec<UnitSnapshot>,
    pub map: Vec<Vec<MapTileSnapshot>>,
}

pub fn snapshot_state(
    unit_query: &Query<(Entity, &Unit, &Position, &Needs, &CurrentState)>,
    map: &GameMap,
) -> GameStateSnapshot {
    let units = unit_query.iter().map(|(_, u, p, n, s)| {
        let mut debuffs = Vec::new();
        if n.food < 25.0 { debuffs.push("hungry".into()); }
        if n.energy < 25.0 { debuffs.push("tired".into()); }
        UnitSnapshot {
            id: u.id, name: u.name.clone(), x: p.x, y: p.y,
            food: n.food, energy: n.energy,
            state: format!("{:?}", s.0), debuffs,
        }
    }).collect();

    let map_data = map.tiles.iter().map(|row| {
        row.iter().map(|tile| {
            let kind = match tile.kind {
                TileKind::Water => "water",
                TileKind::Sand => "sand",
                TileKind::Grass => "grass",
            }.to_string();
            let building = tile.building.as_ref().map(|b| match b {
                Building::BerryBush => "berry_bush",
                Building::Bed => "bed",
            }.to_string());
            MapTileSnapshot { kind, building }
        }).collect()
    }).collect();

    GameStateSnapshot { units, map: map_data }
}
```

- [ ] **Step 3: Create wasm_entry.rs** — WASM exports with `static mut WORLD/MAP`

Use `bevy_ecs::IntoSystem` to run each system. Note: systems are re-initialized each frame (acceptable for MVP — we can cache in a later iteration). The update function returns JSON string with `{ state: GameStateSnapshot, events: Vec<GameEvent> }`.

Key structure:
```
static mut WORLD: Option<World> = None;
static mut MAP: Option<GameMap> = None;

#[wasm_bindgen] pub fn init()
  → Create World, insert resources (Time, GameRng, CommandQueue, EventLog)
  → Create GameMap 30×30
  → Spawn 3 units near center with Needs 80/80
  → Spawn 4 BerryBush + 1 Bed as ECS entities + mark on map tiles

#[wasm_bindgen] pub fn update(dt: f32) -> String
  → Update Time resource
  → Run all 5 systems via IntoSystem
  → Snapshot state via snapshot_state()
  → Drain events from EventLog
  → Return JSON with state + events

#[wasm_bindgen] pub fn send_command(json: &str)
  → Deserialize PlayerCommand from JSON
  → Push to CommandQueue
```

- [ ] **Step 4: Update lib.rs** — add modules:

```rust
pub mod components;
pub mod events;
pub mod game_state;
pub mod map;
pub mod systems;
pub mod wasm_entry;
```

- [ ] **Step 5: Build WASM** — `cd rimworld-mvp/wasm-core && wasm-pack build --target web --out-dir ../web-app/src/wasm`

- [ ] **Step 6: Commit**

```bash
git add rimworld-mvp/wasm-core/src/events.rs rimworld-mvp/wasm-core/src/game_state.rs rimworld-mvp/wasm-core/src/wasm_entry.rs rimworld-mvp/wasm-core/src/lib.rs
git commit -m "feat(w-core): event system, state snapshot, WASM exports"
```

---

### Task 5: Web Worker Bridge

**Files:**
- Create: `rimworld-mvp/web-app/src/worker/game-bridge.ts`

**Interfaces:**
- Consumes: WASM exports (init, update, send_command)
- Produces: postMessage `{ type: 'state_update', payload: { state, events } }`

- [ ] **Step 1: Create game-bridge.ts**

```typescript
/// <reference lib="webworker" />

let wasm: any = null;

async function loadWasm() {
  const wasmModule = await import('../wasm/wasm_core.js');
  await wasmModule.default();
  wasm = wasmModule;
  wasm.init();
}

self.onmessage = async (e: MessageEvent) => {
  const { type, payload } = e.data;
  if (type === 'init') {
    await loadWasm();
    self.postMessage({ type: 'initialized' });
    return;
  }
  if (!wasm) return;
  switch (type) {
    case 'update': {
      const result = wasm.update(payload.dt);
      self.postMessage({ type: 'state_update', payload: JSON.parse(result) });
      break;
    }
    case 'command': {
      wasm.send_command(JSON.stringify(payload.command));
      break;
    }
  }
};
```

- [ ] **Step 2: Commit**

```bash
git add rimworld-mvp/web-app/src/worker/
git commit -m "feat: Web Worker bridge for WASM game core"
```

---

### Task 6: PixiJS Renderer

**Files:**
- Create: `rimworld-mvp/web-app/src/game/constants.ts`
- Create: `rimworld-mvp/web-app/src/game/tilemap.ts`
- Create: `rimworld-mvp/web-app/src/game/sprites.ts`
- Create: `rimworld-mvp/web-app/src/game/renderer.ts`

**Interfaces:**
- Produces: `GameRenderer` class with `init(canvas)`, `render(state)`, `getCanvasCoords(globalX, globalY)`, `destroy()`

- [ ] **Step 1: Create constants.ts**

```typescript
export const TILE_SIZE = 16;
export const MAP_SIZE = 30;
export const CANVAS_WIDTH = MAP_SIZE * TILE_SIZE;  // 480
export const CANVAS_HEIGHT = MAP_SIZE * TILE_SIZE; // 480

export const COLORS = {
  water: 0x3a6ea5, sand: 0xe8d5a3, grass: 0x5a8f4a,
  unit: [0x6dc8f2, 0xf2a86d, 0xa86df2],
  bed: 0x8b4513, berry_bush: 0xcc3333,
  bar_bg: 0x333333, bar_food: 0x44bb44, bar_energy: 0xbbbb44,
};
```

- [ ] **Step 2: Create tilemap.ts**

```typescript
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
```

- [ ] **Step 3: Create sprites.ts**

```typescript
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
```

- [ ] **Step 4: Create renderer.ts**

```typescript
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
```

- [ ] **Step 5: Commit**

```bash
git add rimworld-mvp/web-app/src/game/
git commit -m "feat: PixiJS renderer with tilemap and unit sprites"
```

---

### Task 7: Vue UI Panels

**Files:**
- Create: `rimworld-mvp/web-app/src/store/gameStore.ts`
- Create: `rimworld-mvp/web-app/src/ui/MainPanel.vue`
- Create: `rimworld-mvp/web-app/src/ui/UnitPanel.vue`
- Modify: `rimworld-mvp/web-app/src/App.vue`

- [ ] **Step 1: Create Pinia gameStore.ts**

```typescript
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export interface UnitData {
  id: number; name: string; x: number; y: number;
  food: number; energy: number; state: string; debuffs: string[];
}

export interface GameState {
  units: UnitData[];
  map: { kind: string; building: string | null }[][];
}

export const useGameStore = defineStore('game', () => {
  const gameState = ref<GameState | null>(null);
  const gameSpeed = ref(1);
  const isPaused = ref(false);
  const selectedUnitId = ref<number | null>(null);
  const buildMode = ref<string | null>(null);
  const isInitialized = ref(false);

  const selectedUnit = computed(() => {
    if (selectedUnitId.value === null || !gameState.value) return null;
    return gameState.value.units.find(u => u.id === selectedUnitId.value) ?? null;
  });

  function setState(s: GameState) { gameState.value = s; }
  function setSpeed(s: number) { gameSpeed.value = s; isPaused.value = false; }
  function togglePause() { isPaused.value = !isPaused.value; }
  function selectUnit(id: number | null) { selectedUnitId.value = id; }
  function setBuildMode(m: string | null) { buildMode.value = m; }
  function setInitialized() { isInitialized.value = true; }

  return { gameState, gameSpeed, isPaused, selectedUnitId, selectedUnit, buildMode, isInitialized, setState, setSpeed, togglePause, selectUnit, setBuildMode, setInitialized };
});
```

- [ ] **Step 2: Create MainPanel.vue**

```vue
<template>
  <div class="main-panel">
    <div class="speed-controls">
      <button v-for="s in [1,5,10]" :key="s" :class="{ active: store.gameSpeed === s && !store.isPaused }" @click="store.setSpeed(s)">▶ ×{{ s }}</button>
      <button :class="{ active: store.isPaused }" @click="store.togglePause()">⏸</button>
    </div>
    <div class="build-controls">
      <button :class="{ active: store.buildMode === 'bed' }" @click="store.setBuildMode(store.buildMode === 'bed' ? null : 'bed')">🛏️ Кровать</button>
      <button :class="{ active: store.buildMode === 'berry_bush' }" @click="store.setBuildMode(store.buildMode === 'berry_bush' ? null : 'berry_bush')">🫐 Куст</button>
    </div>
  </div>
</template>
<script setup lang="ts">
import { useGameStore } from '../store/gameStore';
const store = useGameStore();
</script>
<style scoped>
.main-panel { display: flex; gap: 16px; padding: 8px 16px; background: #16213e; border-bottom: 1px solid #0f3460; align-items: center; }
.speed-controls, .build-controls { display: flex; gap: 4px; }
button { padding: 4px 12px; background: #0f3460; color: #eee; border: 1px solid #1a4a8a; cursor: pointer; font-family: monospace; font-size: 14px; }
button:hover { background: #1a4a8a; }
button.active { background: #e94560; border-color: #e94560; }
</style>
```

- [ ] **Step 3: Create UnitPanel.vue**

```vue
<template>
  <div v-if="unit" class="unit-panel">
    <h3>{{ unit.name }}</h3>
    <div class="stat-row"><span>Сытость</span><div class="bar"><div class="bar-fill food" :style="{ width: unit.food + '%' }"></div></div><span class="value">{{ Math.round(unit.food) }}/100</span></div>
    <div class="stat-row"><span>Бодрость</span><div class="bar"><div class="bar-fill energy" :style="{ width: unit.energy + '%' }"></div></div><span class="value">{{ Math.round(unit.energy) }}/100</span></div>
    <div class="state">Состояние: <strong>{{ stateLabel }}</strong></div>
    <div v-if="unit.debuffs.length" class="debuffs">
      <span v-for="d in unit.debuffs" :key="d" class="debuff">{{ d === 'hungry' ? '🍽 Голод!' : '💤 Усталость!' }}</span>
    </div>
  </div>
  <div v-else class="unit-panel empty">Выберите юнита на карте</div>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useGameStore } from '../store/gameStore';
const store = useGameStore();
const unit = computed(() => store.selectedUnit);
const stateLabel = computed(() => {
  if (!unit.value) return '';
  const m: Record<string, string> = { Idle: 'Безделье', Moving: 'Идёт к цели', Eating: 'Ест', Sleeping: 'Спит' };
  return m[unit.value.state] || unit.value.state;
});
</script>
<style scoped>
.unit-panel { width: 240px; padding: 12px; background: #16213e; border-left: 1px solid #0f3460; }
.unit-panel.empty { display: flex; align-items: center; justify-content: center; color: #666; font-style: italic; }
h3 { margin: 0 0 8px; font-size: 16px; }
.stat-row { display: flex; align-items: center; gap: 8px; margin: 4px 0; font-size: 12px; }
.bar { flex: 1; height: 10px; background: #333; border-radius: 2px; overflow: hidden; }
.bar-fill { height: 100%; transition: width 0.3s; }
.bar-fill.food { background: #44bb44; }
.bar-fill.energy { background: #bbbb44; }
.value { width: 50px; text-align: right; }
.state { margin-top: 8px; font-size: 13px; }
.debuffs { margin-top: 4px; }
.debuff { color: #ff4444; font-weight: bold; margin-right: 8px; }
</style>
```

- [ ] **Step 4: Update App.vue** — wire canvas ref + panels

```vue
<template>
  <div id="game-app">
    <MainPanel />
    <div id="game-area">
      <canvas ref="canvasRef" id="game-canvas"></canvas>
      <UnitPanel />
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import MainPanel from './ui/MainPanel.vue';
import UnitPanel from './ui/UnitPanel.vue';
const canvasRef = ref<HTMLCanvasElement | null>(null);
</script>
<style scoped>
#game-app { display: flex; flex-direction: column; height: 100vh; }
#game-area { flex: 1; display: flex; align-items: stretch; justify-content: center; padding: 8px; gap: 8px; }
#game-canvas { image-rendering: pixelated; cursor: crosshair; }
</style>
```

- [ ] **Step 5: Commit**

```bash
git add rimworld-mvp/web-app/src/store/ rimworld-mvp/web-app/src/ui/ rimworld-mvp/web-app/src/App.vue
git commit -m "feat: Vue UI panels with Pinia store, speed/build controls"
```

---

### Task 8: Integration — Game Loop and Input Handling

**Files:**
- Create: `rimworld-mvp/web-app/src/game/engine.ts`
- Modify: `rimworld-mvp/web-app/src/App.vue`

- [ ] **Step 1: Create engine.ts**

```typescript
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
```

- [ ] **Step 2: Update App.vue** — mount engine, remove old static canvas ref logic

```vue
<template>
  <div id="game-app">
    <MainPanel />
    <div id="game-area">
      <canvas ref="canvasRef"></canvas>
      <UnitPanel />
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import MainPanel from './ui/MainPanel.vue';
import UnitPanel from './ui/UnitPanel.vue';
import { GameEngine } from './game/engine';
const canvasRef = ref<HTMLCanvasElement | null>(null);
let engine: GameEngine | null = null;
onMounted(() => { if (canvasRef.value) engine = new GameEngine(canvasRef.value); });
onUnmounted(() => engine?.destroy());
</script>
<style scoped>
#game-app { display: flex; flex-direction: column; height: 100vh; }
#game-area { flex: 1; display: flex; align-items: stretch; justify-content: center; padding: 8px; gap: 8px; }
#game-canvas { image-rendering: pixelated; cursor: crosshair; }
</style>
```

- [ ] **Step 3: Build WASM + run dev**

```bash
cd rimworld-mvp/wasm-core && wasm-pack build --target web --out-dir ../web-app/src/wasm
cd ../web-app && npx vite
```

Expected: tilemap visible, 3 units wandering, speed/pause works, left-click selects unit + moves, build mode places structures.

- [ ] **Step 4: Commit**

```bash
git add rimworld-mvp/web-app/src/game/engine.ts rimworld-mvp/web-app/src/App.vue
git commit -m "feat: game loop, input handling, full integration"
```

---

### Task 9: Final Polish — Edge Cases and Small Fixes

**Files:**
- Modify: `rimworld-mvp/web-app/src/game/engine.ts`

- [ ] **Step 1: Fix sprites.ts** — sleeping body should visually indicate lying down; eating unit shows different color tint

In the sprites render loop, add: if state === `Eating`, draw the body circle with a brighter tint (e.g., `Graphics.tint` or just draw a small food icon). If state === `Sleeping`, draw a horizontal rect.

- [ ] **Step 2: Add full game verification**

Run and test manually:
1. Units wander at ×1, ×5, ×10
2. Pause freezes
3. Click unit → panel shows stats in Russian
4. Left-click tile → selected unit moves there
5. Right-click → deselects
6. Place bed + berry bush via toolbar
7. Units auto-find food when hungry, bed when tired
8. Multiple debuffs display simultaneously

- [ ] **Step 3: Commit**

```bash
git add rimworld-mvp/
git commit -m "feat: final polish — edge cases, animations, verification"
```