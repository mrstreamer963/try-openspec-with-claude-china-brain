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