use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub enum ValueCalculatorRequest {
    Destroy(Entity),
    Initialize(Entity),
}

pub struct ValueCalculatorEventChannelPlugin;

impl Plugin for ValueCalculatorEventChannelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ValueCalculatorRequest>();
    }
}

impl ValueCalculatorRequest {
    pub fn entity(&self) -> Entity {
        match self {
            Self::Destroy(entity) => *entity,
            Self::Initialize(entity) => *entity,
        }
    }
}
