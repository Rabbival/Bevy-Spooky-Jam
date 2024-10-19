use crate::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

pub struct BombExplosionAnimationPlugin;

impl Plugin for BombExplosionAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_explosion_animations.in_set(TickingSystemSet::PostTicking),
        );
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
    for (mut config, mut transform, mut atlas, animation_entity) in &mut query {
        for time_multiplier in &time_multipliers {
            if let TimeMultiplierId::GameTimeMultiplier = time_multiplier.id() {
                let time_to_multiply_delta_in = time_multiplier.value();
                config
                    .frame_timer
                    .tick(time.delta() * time_to_multiply_delta_in as u32);

                if config.frame_timer.just_finished() {
                    if atlas.index == config.last_sprite_index {
                        atlas.index = config.first_sprite_index;
                        if let Some(mut animation_commands) = commands.get_entity(animation_entity)
                        {
                            animation_commands.despawn();
                        }
                    } else {
                        atlas.index += 1;
                        config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                        transform.scale.x += 0.04;
                        transform.scale.y += 0.04;
                    }
                }
            }
        }
    }
}
