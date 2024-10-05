#![allow(clippy::type_complexity)]
pub mod animation;
mod app;
mod common_logic;
mod data_structures;
mod debug;
pub mod ecs;
pub mod game;
mod input;
mod os_access;
pub mod time;
mod trait_unions;

#[macro_use]
mod macros;

#[macro_use]
extern crate lazy_static;

pub mod prelude {
    pub use crate::animation::{translation_change::*, CustomAnimationPlugin};
    pub use crate::app::{
        consts::*, generic_plugins::*, main, main_camera::*, screen_setup::*, tags::*,
    };
    pub use crate::common_logic::{
        argument_validation::*,
        enums::basic_direction::*,
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
        enums::{bevy_log_level::*, log_category::*, os_access_log::*},
        game_session_log::*,
        print_config_struct::*,
        print_log::*,
        print_vec::*,
    };
    pub use crate::ecs::{
        component_utilities::*, despawn_policy::*, entity_error::*, late_despawner::*,
        system_sets::*,
    };
    pub use crate::game::{
        consts::*, event_channels::*, orb::*, patroller::*, player::*, monsters::*, tags::*, world_bounds::*,
        GamePlugin,
    };
    pub use crate::input::{keyboard_input_handler::*, mouse_input_handler::*, InputPlugin};
    pub use crate::os_access::{
        enums::{folder_to_access::*, system_file_type::*},
        folder_access::*,
        os_access_error::*,
        system_file_name::*,
        text_file_access::*,
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
    pub use bevy::{prelude::*, utils::HashMap};
    pub use std::marker::PhantomData;
}
