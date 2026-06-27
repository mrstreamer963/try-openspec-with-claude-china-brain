# RimWorld / StarDeus MVP — Design Spec

## Overview

Browser-based idle colony sim MVP: Rust (bevy_ecs) game core compiled to WASM and running in a Web Worker, with Vite + Vue 3 + PixiJS rendering on the main thread. Three units wander a 30×30 tile map, managing food and energy needs. The player can issue movement orders, place buildings (berry bush, bed), and control game speed.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│  Main Thread                                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────────────────┐  │
│  │ Vue 3 UI │  │ PixiJS   │  │ Game Store           │  │
│  │ (panels) │  │ Canvas   │  │ (reactive state)     │  │
│  └────┬─────┘  └──────────┘  └──────────┬───────────┘  │
│       │         postMessage(state,events)│              │
├───────┴──────────────────────────────────┴──────────────┤
│  Web Worker                                             │
│  ┌────────────────────────────────────────────────────┐ │
│  │ bridge.js: postMessage → wasm_fn → postMessage     │ │
│  └──────────────────────┬─────────────────────────────┘ │
│                         │ wasm_bindgen                   │
│  ┌──────────────────────┴─────────────────────────────┐ │
│  │ Rust Core (bevy_ecs)                               │ │
│  │ Systems: movement, needs, idle AI, player commands │ │
│  │ EventBus → events for UI                           │ │
│  └────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

**Data flow per frame:**
1. `requestAnimationFrame` → calculate dt * game speed
2. `worker.postMessage({ type: "update", dt })`
3. WASM `update(dt)` → systems run → EventBus collects events
4. Worker returns `{ state: FullState, events: Event[] }`
5. PixiJS re-renders tilemap + sprites; Vue updates panels

## Map & Tiles

- Size: 30×30 tiles (constant `MAP_SIZE` as `u16` for future scaling)
- Tile types: `Water` (impassable), `Sand`, `Grass`
- Tile rendering: 16×16 pixel sprites
- Map stored in wasm-core as `Vec<Vec<Tile>>`; tile indices use `u16`

## ECS Components (Rust)

```rust
struct Position { x: f32, y: f32 }
struct Unit { name: String }
struct Needs { food: f32, energy: f32 }
struct Target(Option<(f32, f32)>);

enum UnitState { Idle, Moving { target_x: f32, target_y: f32 }, Eating, Sleeping }

// Marker components for buildings
struct Bed;
struct BerryBush;

struct Tile { kind: TileKind, building: Option<Building> }
enum TileKind { Water, Sand, Grass }
enum Building { BerryBush, Bed }
```

## Systems

1. **NeedDecaySystem** — depletes `food` and `energy` each tick
2. **IdleBehaviorSystem** — no debuff, no player order: pick random walkable tile and move
3. **MovementSystem** — interpolate unit toward target in float coordinates; on arrival → check for building interaction or set Idle
4. **AutoNeedSystem** — if `food < 25` → find nearest berry bush by Euclidean distance and move to eat (restores food); if `energy < 25` → find nearest bed and move to sleep (restores energy)
5. **PlayerCommandSystem** — apply `MoveUnit { id, x, y }` and `PlaceBuilding { x, y, kind }` from the command queue
6. **EventEmitterSystem** — after all systems run, push events to EventBus

## Events (EventBus → UI)

| Event | Fields |
|-------|--------|
| `UnitMoved` | unit_id, from_x, from_y, to_x, to_y |
| `NeedsChanged` | unit_id, food, energy |
| `UnitStateChanged` | unit_id, old_state, new_state |
| `DebuffChanged` | unit_id, debuffs[] (hungry/tired/cleared) |
| `BuildingPlaced` | x, y, kind |

Debuffs are independent: a unit can be both hungry (`food < 25`) and tired (`energy < 25`) simultaneously.

## Player Interaction

- **Select unit:** left-click on unit sprite → panel shows unit stats
- **Move order:** select unit → right-click on tile → unit pathfinds there
- **Cancel order:** right-click on selected unit → clear target → unit reverts to idle wandering
- **Build:** click "Build Bed" or "Build Berry Bush" button in top toolbar (enters placement mode) → left-click on tile → building placed → mode deactivated
- **Pause:** freezes dt; core receives no update tick

## Game Speed Controls

Top toolbar: [▶×1] [▶×5] [▶×10] [⏸]
- dt passed to update() = base_dt * speed_multiplier
- Pause sends dt=0 or skips update call

## UI Panels (Vue 3)

**Top toolbar:** Speed controls + build buttons (bed, berry bush)

**Unit detail panel (right or bottom, visible when a unit is selected):**
- Unit name
- Food bar ████░░ 60/100
- Energy bar ███░ 40/100
- Current state (Idle / Moving / Eating / Sleeping)
- Active debuffs in red: "Hungry!" "Tired!"

## Rendering Layers (PixiJS)

- Layer 0: Tilemap — 16×16 sprites for water, sand, grass
- Layer 1: Buildings — bed sprite, berry bush sprite
- Layer 2: Units — colored sprites (one per unit) with idle/walk/eat/sleep variants
- Layer 3 (overlay on unit): debuff icons and/or colored border

## Project Structure

```
rimworld-mvp/
├── Cargo.toml              # workspace root
├── wasm-core/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # wasm entry, EventBus init, exported functions
│       ├── components.rs   # all ECS components
│       ├── systems/
│       │   ├── needs.rs
│       │   ├── movement.rs
│       │   ├── idle.rs
│       │   ├── auto_need.rs
│       │   ├── player_commands.rs
│       │   └── events.rs
│       ├── map.rs
│       └── events.rs
├── web-app/
│   ├── src/
│   │   ├── worker/
│   │   │   └── game-bridge.ts
│   │   ├── game/
│   │   │   ├── renderer.ts
│   │   │   ├── sprites.ts
│   │   │   └── tilemap.ts
│   │   ├── ui/
│   │   │   ├── MainPanel.vue
│   │   │   └── UnitPanel.vue
│   │   ├── store/
│   │   │   └── gameStore.ts
│   │   └── main.ts
│   ├── index.html
│   └── vite.config.ts
└── package.json
```

## What MVP Does NOT Include

- Unit death
- Combat / damage
- Multiple building types beyond bed and berry bush
- Save/load
- Camera pan/zoom
- Multi-select
- Sound
- Pathfinding beyond simple direct-line movement with walkability check