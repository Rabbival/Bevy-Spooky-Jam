use bevy::{
    color::palettes::css::LIGHT_CORAL,
    math::bounding::{BoundingCircle, BoundingCircleCast},
};
use bevy_light_2d::light::{PointLight2d, PointLight2dBundle};

use crate::prelude::*;

pub struct BombExplosionPreviewerPlugin;

impl Plugin for BombExplosionPreviewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bomb_explosion_preview)
            .add_systems(
                Update,
                (toggle_bomb_explosion_preview, update_explosion_preview)
                    .in_set(InputSystemSet::Handling),
            );
    }
}

fn spawn_bomb_explosion_preview(mut commands: Commands) {
    commands.spawn((
        BombExplosionPreview,
        PointLight2dBundle {
            point_light: PointLight2d {
                color: Color::from(LIGHT_CORAL),
                intensity: 0.0,
                radius: BOMB_EXPLOSION_RADIUS,
                ..default()
            },
            ..default()
        },
    ));
}

fn toggle_bomb_explosion_preview(
    changed_players: Query<(&Player, &Transform), (Changed<Player>, Without<BombExplosionPreview>)>,
    mut explosion_preview_query: Query<
        (&mut PointLight2d, &mut Transform),
        (With<BombExplosionPreview>, Without<Player>),
    >,
    explode_in_contact_query: Query<(&ExplodeInContact, Entity)>,
    mouse_position: Res<CursorWorldPosition>,
) {
    if changed_players.iter().count() == 0 {
        return;
    }
    let mut found_bomb_holder = false;
    for (player, player_transform) in &changed_players {
        if let Some(held_bomb_entity) = player.held_bomb {
            found_bomb_holder = true;
            for (_, mut explosion_preview_transform) in &mut explosion_preview_query {
                update_explosion_preview_location(
                    held_bomb_entity,
                    player_transform.translation.truncate(),
                    mouse_position.0,
                    &mut explosion_preview_transform,
                    &explode_in_contact_query,
                );
            }
            break;
        }
    }
    for (mut explosion_preveiewer_light, _) in &mut explosion_preview_query {
        explosion_preveiewer_light.intensity = if found_bomb_holder {
            EXPLOSION_PREVIEW_LIGHT_INTENSITY
        } else {
            0.0
        };
    }
}

fn update_explosion_preview(
    player_query: Query<(&Player, &Transform), Without<BombExplosionPreview>>,
    mut explosion_preview_query: Query<
        &mut Transform,
        (With<BombExplosionPreview>, Without<Player>),
    >,
    mouse_position: Res<CursorWorldPosition>,
    explode_in_contact_query: Query<(&ExplodeInContact, Entity)>,
    changed_explode_in_contact_compoenents: Query<&ExplodeInContact, Changed<ExplodeInContact>>,
) {
    let players_holding_bomb = player_query
        .iter()
        .filter(|(player, _)| player.held_bomb.is_some());
    if mouse_position.is_changed() || changed_explode_in_contact_compoenents.iter().count() > 0 {
        for (player, player_transform) in players_holding_bomb {
            for mut explosion_preview_transform in &mut explosion_preview_query {
                update_explosion_preview_location(
                    player.held_bomb.unwrap(),
                    player_transform.translation.truncate(),
                    mouse_position.0,
                    &mut explosion_preview_transform,
                    &explode_in_contact_query,
                );
            }
        }
    }
}

fn update_explosion_preview_location(
    held_bomb_entity: Entity,
    player_location: Vec2,
    mouse_location: Vec2,
    explosion_preview_transform: &mut Transform,
    explode_in_contact_query: &Query<(&ExplodeInContact, Entity)>,
) {
    let ray = Ray2d::new(player_location, mouse_location);
    let bomb_bounding_circle = BoundingCircle::new(
        explosion_preview_transform.translation.truncate(),
        BOMB_SIZE,
    );
    let circle_cast = BoundingCircleCast::from_ray(
        bomb_bounding_circle,
        ray,
        player_location.distance(mouse_location),
    );
    let explosion_location = if let Some(collision_point) =
        closest_collision(held_bomb_entity, &circle_cast, explode_in_contact_query)
    {
        collision_point
    } else {
        mouse_location
    };
    explosion_preview_transform.translation = explosion_location.extend(Z_LAYER_BOMB);
}

fn closest_collision(
    held_bomb_entity: Entity,
    circle_cast: &BoundingCircleCast,
    explode_in_contact_query: &Query<(&ExplodeInContact, Entity)>,
) -> Option<Vec2> {
    let mut min_distance = circle_cast.ray.max;
    for circle in explode_in_contact_query
        .iter()
        .filter(|(_, entity)| *entity != held_bomb_entity)
        .map(|(circle, _)| circle.bounding_circle)
    {
        if let Some(intersection_distance) = circle_cast.circle_collision_at(circle) {
            if intersection_distance < min_distance {
                min_distance = intersection_distance;
            }
        }
    }
    if min_distance != circle_cast.ray.max {
        Some(circle_cast.ray.ray.get_point(min_distance))
    } else {
        None
    }
}
