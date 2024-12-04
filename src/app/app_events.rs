use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct PauseToggleRequest;

pub struct AppEventsPlugin;

impl Plugin for AppEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PauseToggleRequest>();
    }
}
