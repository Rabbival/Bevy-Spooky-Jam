use crate::prelude::*;

pub fn remove_component_notify_on_fail<T: Component>(
    entity_to_remove_from: Entity,
    commands: &mut Commands,
) {
    match commands.get_entity(entity_to_remove_from) {
        Some(mut entity_commands) => {
            entity_commands.remove::<T>();
        }
        None => {
            print_error(
                EntityError::CommandsCouldntGetEntity(&format!(
                    "with component: {:?} (component removal attempt)",
                    stringify!(T.type_name).to_string()
                )),
                vec![LogCategory::RequestNotFulfilled],
            );
        }
    }
}

pub fn despawn_recursive_notify_on_fail(
    entity_to_destroy: Entity,
    entity_name: &str,
    commands: &mut Commands,
) {
    match commands.get_entity(entity_to_destroy) {
        Some(entity_commands) => {
            entity_commands.despawn_recursive();
        }
        None => {
            print_error(
                EntityError::CommandsCouldntGetEntity(
                    &(String::from(entity_name) + " (despawn attempt)"),
                ),
                vec![LogCategory::RequestNotFulfilled],
            );
        }
    }
}
