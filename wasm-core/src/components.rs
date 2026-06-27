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