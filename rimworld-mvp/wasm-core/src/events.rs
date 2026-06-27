use bevy_ecs::prelude::*;
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
    pub fn building_placed(x: f32, y: f32, _building: String) -> Self {
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