use crate::sasm::{Action, Processor};

/// Push the current memory size in bytes to the stack.
///
/// Stack:
/// + u32 - Memory size
pub fn memory_size(processor: &mut Processor) -> Action {
    let memory_size = processor.memory().size() as u32;
    unwrap_action!(processor.push_u32(memory_size));
    Action::Ok
}

/// Expands the available memory of the system.
///
/// Stack:
/// - u32 - Increase amount
/// + u32 - Previous size.
///
/// If it fails, the overflow_flag is set.
pub fn memory_grow(processor: &mut Processor) -> Action {
    let increase_amount = unwrap_action!(processor.pop_u32());
    let memory_size = processor.memory().size() as u32;
    let page_size = processor.memory().page_size() as u32;
    let mut pages = increase_amount / page_size;
    if increase_amount % page_size != 0 {
        pages += 1;
    }

    let is_error = processor
        .memory_mut()
        .add_empty_pages(pages as usize)
        .is_err();
    processor.set_overflow_flag(is_error);
    unwrap_action!(processor.push_u32(memory_size));

    Action::Ok
}
