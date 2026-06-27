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