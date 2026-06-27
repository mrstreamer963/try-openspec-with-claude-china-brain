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