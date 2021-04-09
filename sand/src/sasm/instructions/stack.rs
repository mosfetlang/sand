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

/// Add a ?8 const to the stack.
///
/// Stack:
/// + ?8
pub fn const_8(processor: &mut Processor) -> Action {
    let result = unwrap_action!(processor.code_next_u8());
    unwrap_action!(processor.push_u8(result));
    Action::Ok
}

/// Add a ?16 const to the stack.
///
/// Stack:
/// + ?16
pub fn const_16(processor: &mut Processor) -> Action {
    let result = unwrap_action!(processor.code_next_u16());
    unwrap_action!(processor.push_u16(result));
    Action::Ok
}

/// Add a ?32 const to the stack.
///
/// Stack:
/// + ?32
pub fn const_32(processor: &mut Processor) -> Action {
    let result = unwrap_action!(processor.code_next_u32());
    unwrap_action!(processor.push_u32(result));
    Action::Ok
}

/// Add a ?64 const to the stack.
///
/// Stack:
/// + ?64
pub fn const_64(processor: &mut Processor) -> Action {
    let result = unwrap_action!(processor.code_next_u64());
    unwrap_action!(processor.push_u64(result));
    Action::Ok
}
