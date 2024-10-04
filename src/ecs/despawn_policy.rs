#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DespawnPolicy {
    #[default]
    DespawnSelf,
    DespawnSelfAndRemoveFromAffectingTimers,
}
