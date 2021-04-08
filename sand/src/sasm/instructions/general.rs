use crate::sasm::{Action, Processor};

/// Throws a panic signal to the processor finishing the execution.
pub fn unreachable(_: &mut Processor) -> Action {
    Action::Panic("unreachable")
}

/// Does nothing.
pub fn nop(_: &mut Processor) -> Action {
    Action::Ok
}

/// Throws a halt signal to the processor stopping the execution to resume it later.
pub fn debug(_: &mut Processor) -> Action {
    Action::Halt
}
