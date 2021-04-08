use crate::sasm::{Action, Processor};

/// Drops a ?8 from the stack.
///
/// Stack:
/// - ?8
pub fn drop_8(processor: &mut Processor) -> Action {
    unwrap_action!(processor.pop_u8());
    Action::Ok
}

/// Drops a ?16 from the stack.
///
/// Stack:
/// - ?16
pub fn drop_16(processor: &mut Processor) -> Action {
    unwrap_action!(processor.pop_u16());
    Action::Ok
}

/// Drops a ?32 from the stack.
///
/// Stack:
/// - ?32
pub fn drop_32(processor: &mut Processor) -> Action {
    unwrap_action!(processor.pop_u32());
    Action::Ok
}

/// Drops a ?64 from the stack.
///
/// Stack:
/// - ?64
pub fn drop_64(processor: &mut Processor) -> Action {
    unwrap_action!(processor.pop_u64());
    Action::Ok
}
