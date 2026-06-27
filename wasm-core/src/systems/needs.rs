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