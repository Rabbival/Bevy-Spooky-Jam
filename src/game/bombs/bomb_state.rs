#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BombState {
    PreHeld,
    Held,
    Ticking,
}
