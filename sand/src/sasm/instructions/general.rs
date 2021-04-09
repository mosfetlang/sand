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

/// Pops a ?32 value from the stack and jumps to the code position it points to.
/// Can cause a panic when the code position is unavailable.
///
/// Stack:
/// - u32 - Code position.
pub fn branch(processor: &mut Processor) -> Action {
    let code_position = unwrap_action!(processor.pop_u32()) as usize;

    processor.set_program_counter(code_position)?;

    Action::Ok
}

/// Pops ?8 value from the stack that acts as a condition for branching
/// to the next ?32 value and jumps to the code position it points to.
/// Can cause a panic when the code position is unavailable.
///
/// Stack:
/// - u8 - Condition.
/// - u32 - Code position.
pub fn branch_if_8(processor: &mut Processor) -> Action {
    let condition = unwrap_action!(processor.pop_u8());
    let code_position = unwrap_action!(processor.pop_u32()) as usize;

    if condition != 0 {
        processor.set_program_counter(code_position)?;
    }

    Action::Ok
}

/// Pops ?16 value from the stack that acts as a condition for branching
/// to the next ?32 value and jumps to the code position it points to.
/// Can cause a panic when the code position is unavailable.
///
/// Stack:
/// - u16 - Condition.
/// - u32 - Code position.
pub fn branch_if_16(processor: &mut Processor) -> Action {
    let condition = unwrap_action!(processor.pop_u16());
    let code_position = unwrap_action!(processor.pop_u32()) as usize;

    if condition != 0 {
        processor.set_program_counter(code_position)?;
    }

    Action::Ok
}

/// Pops ?32 value from the stack that acts as a condition for branching
/// to the next ?32 value and jumps to the code position it points to.
/// Can cause a panic when the code position is unavailable.
///
/// Stack:
/// - u32 - Condition.
/// - u32 - Code position.
pub fn branch_if_32(processor: &mut Processor) -> Action {
    let condition = unwrap_action!(processor.pop_u32());
    let code_position = unwrap_action!(processor.pop_u32()) as usize;

    if condition != 0 {
        processor.set_program_counter(code_position)?;
    }

    Action::Ok
}

/// Pops ?64 value from the stack that acts as a condition for branching
/// to the next ?32 value and jumps to the code position it points to.
/// Can cause a panic when the code position is unavailable.
///
/// Stack:
/// - u64 - Condition.
/// - u32 - Code position.
pub fn branch_if_64(processor: &mut Processor) -> Action {
    let condition = unwrap_action!(processor.pop_u64());
    let code_position = unwrap_action!(processor.pop_u32()) as usize;

    if condition != 0 {
        processor.set_program_counter(code_position)?;
    }

    Action::Ok
}
