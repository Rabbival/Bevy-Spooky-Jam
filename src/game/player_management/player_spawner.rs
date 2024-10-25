use crate::{prelude::*, read_no_field_variant};

pub struct PlayerSpawnerPlugin;

impl Plugin for PlayerSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            respawn_player_on_game_restart.in_set(GameRestartSystemSet::Spawning),
        );
    }
}

fn respawn_player_on_game_restart(
    mut event_reader: EventReader<GameEvent>,
    sprites_atlas_resource: Res<PlayerSpritesAtlas>,
    player_input_map: Res<PlayerInputMap>,
    commands: Commands,
) {
    if read_no_field_variant!(event_reader, GameEvent::RestartGame).count() > 0 {
        spawn_player(sprites_atlas_resource, player_input_map, commands);
    }
}

fn spawn_player(
    sprites_atlas_resource: Res<PlayerSpritesAtlas>,
    player_input_map: Res<PlayerInputMap>,
    mut commands: Commands,
) {
    let input_map = player_input_map.0.clone();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(48.0, 64.0)),
                ..default()
            },
            texture: sprites_atlas_resource.image_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, Z_LAYER_PLAYER),
            ..default()
        },
        TextureAtlas {
            layout: sprites_atlas_resource.atlas_handle.clone(),
            index: 1,
        },
        InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        },
        AffectingTimerCalculators::default(),
        Player::default(),
        BombAffected::default(),
        WorldBoundsWrapped,
        PlayerMonsterCollider::new(PLAYER_COLLIDER_RADIUS),
    ));
}
