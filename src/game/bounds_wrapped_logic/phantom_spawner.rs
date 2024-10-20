use bevy_light_2d::light::PointLight2d;

use crate::prelude::*;

pub struct PhantomSpawnerPlugin;

impl Plugin for PhantomSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attach_phantom_children_to_newborn_bounds_wrapped);
    }
}

fn attach_phantom_children_to_newborn_bounds_wrapped(
    newborn_wrapped_query: Query<
        (
            Entity,
            &Transform,
            &Sprite,
            &Handle<Image>,
            Option<&Player>,
            Option<&Bomb>,
            Option<&Monster>,
            Option<&Children>,
        ),
        Added<WorldBoundsWrapped>,
    >,
    bomb_children_query: Query<(&Text, &PointLight2d), With<Parent>>,
    mut commands: Commands,
) {
    for (
        entity,
        transform,
        sprite,
        texture,
        maybe_player,
        maybe_bomb,
        maybe_monster,
        maybe_children,
    ) in &newborn_wrapped_query
    {
        let mut phantom_children = vec![];
        let locations_relative_to_parent =
            BoundsWrappedPhantom::relative_location_to_parent(transform.translation.truncate())
                .map(|two_d_location| two_d_location.extend(0.0));
        for index in 0..BOUNDS_WRAPPED_PHANTOMS_PER_PARENT {
            if let Some(relative_location) = locations_relative_to_parent.get(index) {
                let raw_child_entity =
                    spawn_phantom_child(index, relative_location, sprite, texture, &mut commands);
                let full_child_entity = enrich_phantom_child(
                    raw_child_entity,
                    maybe_player,
                    maybe_bomb,
                    maybe_monster,
                    maybe_children,
                    &bomb_children_query,
                    &mut commands,
                );
                phantom_children.push(full_child_entity);
            }
        }
        commands.entity(entity).push_children(&phantom_children);
    }
}

fn spawn_phantom_child(
    index: usize,
    location_relative_to_parent: &Vec3,
    parent_sprite: &Sprite,
    parent_texture: &Handle<Image>,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn((
            BoundsWrappedPhantom { index },
            SpriteBundle {
                sprite: parent_sprite.clone(),
                texture: parent_texture.clone(),
                transform: Transform::from_translation(*location_relative_to_parent),
                ..default()
            },
        ))
        .id()
}

fn enrich_phantom_child(
    child_entity: Entity,
    maybe_player: Option<&Player>,
    maybe_bomb: Option<&Bomb>,
    maybe_monster: Option<&Monster>,
    maybe_original_children: Option<&Children>,
    bomb_children_query: &Query<(&Text, &PointLight2d), With<Parent>>,
    commands: &mut Commands,
) -> Entity {
    if let Some(player) = maybe_player {
        commands.entity(child_entity).insert(player.clone());
    } else if let Some(bomb) = maybe_bomb {
        commands
            .entity(child_entity)
            .insert((bomb.clone(), BombTag));
    } else if let Some(monster) = maybe_monster {
        commands.entity(child_entity).insert(monster.clone());
    }
    if let Some(original_children) = maybe_original_children {
        for original_child in original_children {
            if let Ok((text, light)) = bomb_children_query.get(*original_child) {
                let grandson = commands
                    .spawn((
                        Text2dBundle {
                            text: text.clone(),
                            ..default()
                        },
                        light.clone(),
                    ))
                    .id();
                commands.entity(child_entity).add_child(grandson);
            }
        }
    }
    child_entity
}
