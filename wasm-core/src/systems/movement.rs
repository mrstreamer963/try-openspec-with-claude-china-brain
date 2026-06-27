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
            if travel >= dist {
                pos.x = target_pos.0;
                pos.y = target_pos.1;
                target.0 = None;
                let gx = pos.x.floor() as u16;
                let gy = pos.y.floor() as u16;
                let building = map.get_tile(gx, gy).and_then(|t| t.building.as_ref());
                match building {
                    Some(Building::BerryBush) => state.0 = UnitState::Eating,
                    Some(Building::Bed) => state.0 = UnitState::Sleeping,
                    None => state.0 = UnitState::Idle,
                }
            } else {
                pos.x += (dx / dist) * travel;
                pos.y += (dy / dist) * travel;
                state.0 = UnitState::Moving;
            }
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