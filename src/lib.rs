#![allow(clippy::type_complexity)]
pub mod animation;
mod app;
mod audio;
mod common_logic;
mod data_structures;
mod debug;
pub mod ecs;
pub mod game;
mod input;
pub mod time;
mod trait_unions;
mod ui;

#[macro_use]
mod macros;

#[macro_use]
extern crate lazy_static;

pub mod prelude {
    pub use crate::animation::{
        bomb_explosion_animation::*, color_change::*, consts::*, dynamic_light_manager::*,
        frame_change::*, frame_sequence::*, scale_change::*, translation_change::*,
        CustomAnimationPlugin,
    };
    pub use crate::app::{
        app_events::*, app_state::*, app_state_manager::*, assets_loader::*, consts::*,
        generic_plugins::*, main, main_camera::*, screen_setup::*, tags::*,
    };
    pub use crate::audio::{
        consts::*, music_player::*, sound_event_channel::*, sound_player::*, tags::*,
        GameAudioPlugin,
    };
    pub use crate::common_logic::{
        argument_validation::*,
        enums::basic_direction::*,
        float_calculations::*,
        interpolation::{interpolator::*, value_by_interpolation::*},
        mismatch_error::*,
        movement_type::*,
    };
    pub use crate::data_structures::{
        path_travel_type::*,
        vec_based::{vec_based_array::*, vec_based_array_error::*},
    };
    pub use crate::debug::{
        consts::*,
        enums::{bevy_log_level::*, functionality_override::*, log_category::*},
        gizmos::{
            player_monster_collision_gizmos::*, range_gizmos::*, ray_gizmos::*, GizmosPlugin,
        },
        print_config_struct::*,
        print_log::*,
        print_vec::*,
        DebugPlugin,
    };
    pub use crate::ecs::{
        component_utilities::*,
        entity_error::*,
        enums::{despawn_policy::*, spawn_request_type::*},
        late_despawner::*,
        system_sets::*,
    };
    pub use crate::game::{
        bombs::{
            bomb::*, bomb_error::*, bomb_events::*, bomb_explosion_previewer::*, bomb_picking::*,
            bomb_spawner::*, bomb_spawning_sequence_manager::*, bomb_state::*, bomb_throwing::*,
            bomb_ticker::*, consts::*, explode_in_contact_manager::*, explosion_manager::*,
            tags::*, BombsPlugin,
        },
        bounds_wrapped_logic::{
            bounds_wrapped_phantom::*, calculate_reach_beyond_screen_border, consts::*,
            phantom_manager::*, phantom_spawner::*, BoundsWrappedLogicPlugin,
        },
        consts::*,
        event_channels::*,
        floor_spawner::*,
        game_event_handler::*,
        monsters::{
            consts::*,
            monster::*,
            monster_audio::*,
            monster_error::*,
            monster_events::*,
            monster_path::{
                main_path_initiation::*, stray_path_ender::*, stray_path_updater::*,
                MonsterPathUpdatingPlugin,
            },
            monster_spawner::*,
            monster_spawning_sequence_manager::*,
            state_management::{
                monster_state::*, monster_state_changer::*, MonsterStateManagementPlugin,
            },
            visuals::{animation_manager::*, state_change_visualizer::*, MonsterVisualsPlugin},
            MonstersPlugin,
        },
        player_management::{
            consts::*, player_event_channel::*, player_monster_collision_detection::*,
            player_movement::*, player_spawner::*, player_visuals::*, tags::*, PlayerPlugin,
        },
        respawner::*,
        scores::{components::*, score_event_channel::*, score_manager::*, ScorePlugin},
        tags::*,
        GamePlugin,
    };
    pub use crate::input::{
        enums::{player_action::*, ui_action::*},
        input_maps::{player_input_map::*, ui_input_map::*, InputMapsPlugin},
        mouse_input_handler::*,
        player_input::*,
        ui_input::*,
        InputPlugin,
    };
    pub use crate::time::{
        affecting_timer_calculators_management::{
            affecting_timer_calculators::*, affecting_timer_calculators_manager::*,
            timer_calculator_set_policy::*,
        },
        consts::*,
        emitting_timer::*,
        errors::{
            time_related_error::*, timer_affected_entities_error::*, timer_sequence_error::*,
        },
        events::{
            calculate_and_send_going_event::*, remove_from_timer_affected_entities::*,
            set_time_multiplier::*, timer_done_event::*, timer_fire_request::*,
            timer_going_event::*, update_affected_entity_after_timer_birth::*,
            value_calculator_event_channel::*, TimeEventChannelPlugin,
        },
        going_event_management::{
            going_event_emitting::*, going_event_value_calculator::*,
            going_event_value_calculators_plugin::*,
        },
        time_multiplication::{
            time_multiplier::*, time_multiplier_id::*, time_multiplier_management::*,
            TimeMutiplicationPlugin,
        },
        timer_affected_entity::*,
        timer_and_calculator::*,
        timer_management::{
            timer_affected_entities_change::*, timer_clearing::*, timer_firing::*,
            timer_ticking::*, TimerManagementPlugin,
        },
        timer_sequencing::{
            timer_parent_sequence::*, timer_sequence::*, timer_sequence_manager::*,
            timer_sequence_status::*,
        },
        TimePlugin,
    };
    pub use crate::trait_unions::*;
    pub use crate::ui::{consts::*, ui_spawner::*, ui_updater::*, UiPlugin};
    pub use bevy::{prelude::*, utils::HashMap};
    pub use leafwing_input_manager::prelude::*;
    pub use std::marker::PhantomData;
}
