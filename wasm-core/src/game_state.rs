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
    world: &mut World,
    map: &GameMap,
) -> GameStateSnapshot {
    let mut unit_query = world.query::<(Entity, &Unit, &Position, &Needs, &CurrentState)>();
    let units = unit_query.iter(world).map(|(_, u, p, n, s)| {
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