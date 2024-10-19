use crate::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u32,
    pub time_since_frame_update: f32,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u32) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            time_since_frame_update: 0.0,
        }
    }
}

pub struct BombExplosionAnimationPlugin;

impl Plugin for BombExplosionAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (listen_for_exploded_bombs, update_explosion_animations)
                .chain()
                .in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn listen_for_exploded_bombs(
    mut explosions_listener: EventReader<BombExploded>,
    bomb_explosion_sprites_atlas_resource: Res<BombExplosionSpritesAtlas>,
    mut commands: Commands,
) {
    for explosion in explosions_listener.read() {
        let mut rng = rand::thread_rng();
        let animation_config = AnimationConfig::new(0, 60, BOMB_EXPLOSION_ANIMATION_FPS);
        commands.spawn((
            SpriteBundle {
                texture: bomb_explosion_sprites_atlas_resource.image_handle.clone(),
                transform: Transform::from_xyz(
                    explosion.location.x,
                    explosion.location.y,
                    Z_LAYER_BOMB_EXPLOSION,
                )
                .with_rotation(Quat::from_rotation_z(rng.gen_range(0.0..360.0)))
                .with_scale(Vec3::new(2.5, 2.5, 0.0)),
                ..default()
            },
            TextureAtlas {
                layout: bomb_explosion_sprites_atlas_resource.atlas_handle.clone(),
                index: animation_config.first_sprite_index,
            },
            animation_config,
            WorldBoundsWrapped,
        ));
    }
}

fn update_explosion_animations(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationConfig,
        &mut Transform,
        &mut TextureAtlas,
        Entity,
    )>,
    time_multipliers: Query<&TimeMultiplier>,
    mut commands: Commands,
) {
    for (mut animation_config, mut animation_transform, mut atlas, animation_entity) in &mut query {
        for time_multiplier in &time_multipliers {
            if let TimeMultiplierId::GameTimeMultiplier = time_multiplier.id() {
                animation_config.time_since_frame_update +=
                    time.delta_seconds() * (time_multiplier.value().powf(2.0));
                let frames_since_frame_update =
                    (animation_config.time_since_frame_update * animation_config.fps as f32) as u32;
                animation_config.time_since_frame_update -=
                    (frames_since_frame_update / BOMB_EXPLOSION_ANIMATION_FPS) as f32;
                for _frame in 0..frames_since_frame_update {
                    if atlas.index == animation_config.last_sprite_index {
                        if let Some(mut animation_commands) = commands.get_entity(animation_entity)
                        {
                            animation_commands.despawn();
                            break;
                        }
                    } else {
                        atlas.index += 1;
                        animation_transform.scale.x += 0.04;
                        animation_transform.scale.y += 0.04;
                    }
                }
                break;
            }
        }
    }
}
