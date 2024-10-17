use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemSet {
    ListeningPreperations,
    Listening,
    Handling,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum TickingSystemSet {
    PreTickingEarlyPreperations,
    PreTickingPreperations,
    PreTicking,
    TimerTicking,
    PostTickingImmidiate,
    PostTicking,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum EndOfFrameSystemSet {
    PreTimerClearing,
    TimerClearing,
    LateDespawn,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MonsterSystemSet {
    StateChanging,
    PathAndVisualUpdating,
    PostPathUpdating,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameRestartSystemSet {
    Despawning,
    Respawning,
}

pub struct SystemSetsPlugin;

impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                (
                    InputSystemSet::ListeningPreperations,
                    InputSystemSet::Listening,
                    InputSystemSet::Handling,
                )
                    .chain(),
                (
                    GameRestartSystemSet::Despawning,
                    GameRestartSystemSet::Respawning,
                )
                    .chain()
                    .after(InputSystemSet::Handling),
                (
                    MonsterSystemSet::StateChanging,
                    MonsterSystemSet::PathAndVisualUpdating,
                    MonsterSystemSet::PostPathUpdating,
                )
                    .chain()
                    .after(InputSystemSet::Handling)
                    .before(TickingSystemSet::PreTickingEarlyPreperations),
                (
                    TickingSystemSet::PreTickingEarlyPreperations,
                    TickingSystemSet::PreTickingPreperations,
                    TickingSystemSet::PreTicking,
                    TickingSystemSet::TimerTicking,
                    TickingSystemSet::PostTickingImmidiate,
                    TickingSystemSet::PostTicking,
                )
                    .chain()
                    .after(InputSystemSet::Handling),
                (
                    EndOfFrameSystemSet::PreTimerClearing,
                    EndOfFrameSystemSet::TimerClearing,
                    EndOfFrameSystemSet::LateDespawn,
                )
                    .chain()
                    .after(TickingSystemSet::PostTicking),
            ),
        );
    }
}
