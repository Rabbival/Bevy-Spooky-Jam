#[derive(Debug, Clone, Copy)]
pub enum TimerCalculatorSetPolicy {
    KeepNewTimer,
    IgnoreNewIfAssigned,
    AppendToTimersOfType,
}
