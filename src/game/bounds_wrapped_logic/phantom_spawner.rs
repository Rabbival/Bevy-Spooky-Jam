use bevy::color::palettes::css::CRIMSON;
use bevy_light_2d::light::{PointLight2d, PointLight2dBundle};

use crate::prelude::*;

pub struct PhantomSpawnerPlugin;

impl Plugin for PhantomSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attach_phantom_lights_to_newborn_monsters);
    }
}

fn attach_phantom_lights_to_newborn_monsters(
    newborn_wrapped_monsters: Query<
        (Entity, &Transform, &PointLight2d),
        (Added<WorldBoundsWrapped>, With<Monster>),
    >,
    mut commands: Commands,
) {
    for (monster_entity, monster_transform, monster_light) in &newborn_wrapped_monsters {
        let mut phantom_children = vec![];
        let locations_relative_to_parent = BoundsWrappedPhantom::relative_location_to_parent(
            monster_transform.translation.truncate(),
        )
        .map(|two_d_location| two_d_location.extend(0.0));
        for phantom_index in 0..BOUNDS_WRAPPED_PHANTOMS_PER_PARENT {
            if let Some(relative_location) = locations_relative_to_parent.get(phantom_index) {
                let phantom_light_entity = spawn_phantom_light(
                    phantom_index,
                    monster_light,
                    relative_location,
                    &mut commands,
                );
                phantom_children.push(phantom_light_entity);
            }
        }
        commands
            .entity(monster_entity)
            .push_children(&phantom_children);
    }
}

fn spawn_phantom_light(
    phantom_index: usize,
    monster_light: &PointLight2d,
    relative_location: &Vec3,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn((
            PointLight2dBundle {
                point_light: PointLight2d {
                    intensity: monster_light.intensity,
                    color: Color::from(CRIMSON),
                    radius: monster_light.radius,
                    falloff: MONSTER_PHANTOM_LIGHT_FALLOFF,
                    ..default()
                },
                transform: Transform::from_translation(*relative_location),
                ..default()
            },
            BoundsWrappedPhantom {
                index: phantom_index,
            },
        ))
        .id()
}
