use wasm_bindgen::prelude::*;
use bevy_ecs::prelude::*;
use crate::{
    components::*,
    events::*,
    game_state::*,
    map::GameMap,
    systems::{
        needs::need_decay_system,
        movement::movement_system,
        idle::idle_behavior_system,
        auto_need::auto_need_system,
        player_commands::{player_command_system, CommandQueue, BuildingKind},
    },
};

static mut WORLD: Option<World> = None;

#[wasm_bindgen]
pub fn init() {
    let mut world = World::new();
    world.insert_resource(Time { delta_seconds: 0.016 });
    world.insert_resource(GameRng(12345));
    world.insert_resource(CommandQueue::default());
    world.insert_resource(EventLog::default());

    let center = MAP_SIZE as f32 / 2.0;
    let names = ["Starlight", "Ember", "Rust"];
    for i in 0..3 {
        let offset_x = (i as f32 - 1.0) * 1.5;
        let offset_y = ((i as f32 * 2.0) % 3.0 - 1.0) * 1.5;
        world.spawn((
            Unit { id: i as u32, name: names[i as usize].to_string() },
            Position { x: center + offset_x, y: center + offset_y },
            Needs { food: 80.0, energy: 80.0 },
            Target(None),
            CurrentState(UnitState::Idle),
            Speed(1.0),
        ));
    }

    // Insert map into World as a Resource so systems can access it via Res<GameMap>
    world.insert_resource(GameMap::new(MAP_SIZE));
    // Place buildings on the map
    {
        let mut wmap = world.resource_mut::<GameMap>();
        for i in 0..4 {
            let bx = 5.0 + (i as f32 * 7.0);
            let by = 25.0;
            if let Some(t) = wmap.get_tile_mut(bx as u16, by as u16) {
                t.building = Some(Building::BerryBush);
            }
        }
        if let Some(t) = wmap.get_tile_mut(25, 5) {
            t.building = Some(Building::Bed);
        }
    }

    // Spawn building entities for ECS queries (auto_need_system, etc.)
    for i in 0..4 {
        let bx = 5.0 + (i as f32 * 7.0);
        let by = 25.0;
        world.spawn((Position { x: bx, y: by }, BerryBush));
    }
    world.spawn((Position { x: 25.0, y: 5.0 }, Bed));

    unsafe {
        WORLD = Some(world);
    }
}

#[wasm_bindgen]
pub fn update(dt: f32) -> String {
    let world = unsafe { WORLD.as_mut().expect("World not initialized") };

    world.insert_resource(Time { delta_seconds: dt });

    let mut need_sys = IntoSystem::into_system(need_decay_system);
    need_sys.initialize(world);
    need_sys.run((), world);

    let mut auto_sys = IntoSystem::into_system(auto_need_system);
    auto_sys.initialize(world);
    auto_sys.run((), world);

    let mut idle_sys = IntoSystem::into_system(idle_behavior_system);
    idle_sys.initialize(world);
    idle_sys.run((), world);

    let mut move_sys = IntoSystem::into_system(movement_system);
    move_sys.initialize(world);
    move_sys.run((), world);

    let mut cmd_sys = IntoSystem::into_system(player_command_system);
    cmd_sys.initialize(world);
    cmd_sys.run((), world);

    let state = snapshot_state(world);

    let mut event_log = world.resource_mut::<EventLog>();
    let events = event_log.drain_all();

    let output = serde_json::json!({
        "state": state,
        "events": events,
    });
    output.to_string()
}

#[wasm_bindgen]
pub fn send_command(json: &str) {
    let world = unsafe { WORLD.as_mut().expect("World not initialized") };
    let cmd: serde_json::Value = serde_json::from_str(json).expect("Invalid command");
    let mut queue = world.resource_mut::<CommandQueue>();

    if let Some(command_obj) = cmd.as_object() {
        if let Some(move_cmd) = command_obj.get("MoveUnit") {
            let unit_id = move_cmd["unit_id"].as_f64().unwrap_or(0.0) as u32;
            let x = move_cmd["x"].as_f64().unwrap_or(0.0) as f32;
            let y = move_cmd["y"].as_f64().unwrap_or(0.0) as f32;
            queue.commands.push(super::systems::player_commands::PlayerCommand::MoveUnit { unit_id, x, y });
        } else if let Some(build_cmd) = command_obj.get("PlaceBuilding") {
            let x = build_cmd["x"].as_f64().unwrap_or(0.0) as u16;
            let y = build_cmd["y"].as_f64().unwrap_or(0.0) as u16;
            let kind = build_cmd["kind"].as_str().unwrap_or("");
            let kind = match kind {
                "bed" => BuildingKind::Bed,
                _ => BuildingKind::BerryBush,
            };
            queue.commands.push(super::systems::player_commands::PlayerCommand::PlaceBuilding { x, y, kind });
        }
    }
}