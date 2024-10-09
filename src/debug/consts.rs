use crate::prelude::*;

pub const GAME_SESSION_LOG_FILE_NAME: &str = "latest_game_session_log";

lazy_static! {
    pub static ref LOG_CATEGORYS_TO_PRINT: Vec<LogCategory> = vec![
        LogCategory::Crucial,
        LogCategory::RequestNotFulfilled,
        LogCategory::Monster,
    ];
    pub static ref LOG_CATEGORYS_TO_APPEND_TO_SESSION_LOG: Vec<LogCategory> =
        vec![LogCategory::Crucial];
    pub static ref LOG_LEVELS_TO_PRINT: Vec<BevyLogLevel> = vec![
        BevyLogLevel::Error,
        BevyLogLevel::Warning,
        BevyLogLevel::Info
    ];
    pub static ref ACTIVE_FUNCTIONALITY_OVERRIDES: Vec<FunctionalityOverride> = vec![
        // FunctionalityOverride::AlwaysSpawnBombsInMiddle,
        // FunctionalityOverride::PlayerMayCarryInfiniteBombs
        // FunctionalityOverride::AllBombsExplodeAfterOneSecond,
        // FunctionalityOverride::EnemiesDontMove
    ];
}
