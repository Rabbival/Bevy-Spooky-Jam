use crate::{prelude::*, read_single_field_variant, return_if_at_limit};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub struct OrbPlugin;

impl Plugin for OrbPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_orb, collect_all_orbs).in_set(InputSystemSet::Handling),
        );
    }
}

pub fn spawn_orb(
    mut spawn_request_reader: EventReader<OrbEvent>,
    orb_query: Query<&Orb>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for requested_spawn_location in
        read_single_field_variant!(spawn_request_reader, OrbEvent::SpawnOrb)
    {
        return_if_at_limit!(orb_query, ORB_MAX_COUNT);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(ORB_MAX_RADIUS))),
                material: materials.add(Color::srgb(0.6, 0.1, 0.0)),
                transform: Transform::from_xyz(
                    requested_spawn_location.x,
                    requested_spawn_location.y,
                    0.0,
                ),
                ..default()
            },
            AffectingTimerCalculators::default(),
            Orb,
        ));
    }
}

pub fn collect_all_orbs(
    mut event_reader: EventReader<OrbEvent>,
    mut event_writer: EventWriter<TimerFireRequest>,
    orb_query: Query<(&Transform, Entity), With<Orb>>,
    mut commands: Commands,
) {
    for orb_collection_target in read_single_field_variant!(event_reader, OrbEvent::CollectAllOrbs)
    {
        let timer_affected_entities =
            get_orbs_and_calculators_for_timer(&orb_query, orb_collection_target, &mut commands);
        event_writer.send(TimerFireRequest {
            timer: EmittingTimer::new(
                timer_affected_entities,
                vec![TimeMultiplierId::GameTimeMultiplier],
                ORB_COLLECTION_TIME,
                TimerDoneEventType::DespawnAffectedEntities(
                    DespawnPolicy::DespawnSelfAndRemoveFromAffectingTimers,
                ),
            ),
            parent_sequence: None,
        });
    }
}

fn get_orbs_and_calculators_for_timer(
    orb_query: &Query<(&Transform, Entity), With<Orb>>,
    orb_collection_target: &Vec2,
    commands: &mut Commands,
) -> Vec<TimerAffectedEntity> {
    let mut timer_affected_entities = vec![];
    for (orb_transform, orb_entity) in orb_query {
        let value_calculator_entity = commands
            .spawn(GoingEventValueCalculator::new(
                TimerCalculatorSetPolicy::IgnoreNewIfAssigned,
                ValueByInterpolation::from_goal_and_current(
                    orb_transform.translation,
                    Vec3::from((*orb_collection_target, 0.0)),
                    Interpolator::new(ORB_COLLECTION_POWER),
                ),
                TimerGoingEventType::Move(MovementType::InDirectLine),
            ))
            .id();
        timer_affected_entities.push(TimerAffectedEntity {
            affected_entity: orb_entity,
            value_calculator_entity: Some(value_calculator_entity),
        });
    }
    timer_affected_entities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_orb_limitation() {
        let mut app = App::new();
        let more_than_max = ORB_MAX_COUNT + 1;
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<ColorMaterial>>()
            .add_event::<OrbEvent>()
            .add_systems(Update, (send_orb_event, spawn_orb).chain());

        for _ in 0..more_than_max {
            app.update();
        }

        assert_eq!(
            app.world_mut().query::<&Orb>().iter(app.world()).len(),
            ORB_MAX_COUNT
        );
    }

    fn send_orb_event(mut event_writer: EventWriter<OrbEvent>) {
        event_writer.send(OrbEvent::SpawnOrb(Vec2::default()));
    }
}
