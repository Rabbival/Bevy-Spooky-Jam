use bevy_light_2d::light::PointLight2d;

use crate::prelude::*;

pub struct PhantomManagerPlugin;

impl Plugin for PhantomManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_phantom_objects_screens,
                update_monster_phantom_lights,
            ),
        );
    }
}

fn update_phantom_objects_screens(
    mut phantom_transforms: Query<
        (&mut Transform, &Parent, &BoundsWrappedPhantom),
        Without<WorldBoundsWrapped>,
    >,
    changed_bounds_wrapped_transforms: Query<
        (&Transform, Entity),
        (With<WorldBoundsWrapped>, Changed<Transform>),
    >,
) {
    for (parent_transform, parent_entity) in &changed_bounds_wrapped_transforms {
        for (mut phantom_transform, parent, bound_wrapped_phantom_component) in
            &mut phantom_transforms
        {
            if parent.get() == parent_entity {
                let locations_relative_to_parent =
                    BoundsWrappedPhantom::relative_location_to_parent(
                        parent_transform.translation.truncate(),
                    );
                if let Some(new_transform) =
                    locations_relative_to_parent.get(bound_wrapped_phantom_component.index)
                {
                    phantom_transform.translation =
                        new_transform.extend(phantom_transform.translation.z);
                }
            }
        }
    }
}

fn update_monster_phantom_lights(
    changed_monster_lights: Query<
        (&PointLight2d, &Children),
        (
            Changed<PointLight2d>,
            With<Monster>,
            Without<BoundsWrappedPhantom>,
        ),
    >,
    mut phantom_lights: Query<&mut PointLight2d, With<BoundsWrappedPhantom>>,
) {
    for (monster_light, monster_children) in &changed_monster_lights {
        for monster_child in monster_children {
            if let Ok(mut phantom_light) = phantom_lights.get_mut(*monster_child) {
                phantom_light.intensity = monster_light.intensity;
            }
        }
    }
}
