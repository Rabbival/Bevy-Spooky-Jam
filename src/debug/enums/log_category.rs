#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogCategory {
    Crucial,
    OsAccess,
    ValueValidation,
    RequestNotFulfilled,
    Time,
    Player,
    Monster,
    Bomb,
}
