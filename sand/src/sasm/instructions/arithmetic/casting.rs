use crate::sasm::{Action, Processor};

/// Pops a ?8 value from the stack, extends it to a u16 value.
///
/// Stack:
/// - ?8 - Input value.
/// + u16 - Output value.
pub fn extend_8_to_16(processor: &mut Processor) -> Action {
    let input_value = unwrap_action!(processor.pop_u8());
    let output_value = input_value as u16;

    unwrap_action!(processor.push_u16(output_value));

    Action::Ok
}

/// Pops a ?8 value from the stack, extends it to a u32 value.
///
/// Stack:
/// - ?8 - Input value.
/// + u32 - Output value.
pub fn extend_8_to_32(processor: &mut Processor) -> Action {
    let input_value = unwrap_action!(processor.pop_u8());
    let output_value = input_value as u32;

    unwrap_action!(processor.push_u32(output_value));

    Action::Ok
}

/// Pops a ?16 value from the stack, extends it to a u32 value.
///
/// Stack:
/// - ?16 - Input value.
/// + u32 - Output value.
pub fn extend_16_to_32(processor: &mut Processor) -> Action {
    let input_value = unwrap_action!(processor.pop_u16());
    let output_value = input_value as u32;

    unwrap_action!(processor.push_u32(output_value));

    Action::Ok
}

/// Pops a ?8 value from the stack, extends it to a u64 value.
///
/// Stack:
/// - ?8 - Input value.
/// + u64 - Output value.
pub fn extend_8_to_64(processor: &mut Processor) -> Action {
    let input_value = unwrap_action!(processor.pop_u8());
    let output_value = input_value as u64;

    unwrap_action!(processor.push_u64(output_value));

    Action::Ok
}

/// Pops a ?16 value from the stack, extends it to a u64 value.
///
/// Stack:
/// - ?16 - Input value.
/// + u64 - Output value.
pub fn extend_16_to_64(processor: &mut Processor) -> Action {
    let input_value = unwrap_action!(processor.pop_u16());
    let output_value = input_value as u64;

    unwrap_action!(processor.push_u64(output_value));

    Action::Ok
}

/// Pops a ?32 value from the stack, extends it to a u64 value.
///
/// Stack:
/// - ?32 - Input value.
/// + u64 - Output value.
pub fn extend_32_to_64(processor: &mut Processor) -> Action {
    let input_value = unwrap_action!(processor.pop_u32());
    let output_value = input_value as u64;

    unwrap_action!(processor.push_u64(output_value));

    Action::Ok
}

/// Pops a ?8 value from the stack, extends it to a i16 value.
///
/// Stack:
/// - ?8 - Input value.
/// + i16 - Output value.
pub fn extend_sign_8_to_16(processor: &mut Processor) -> Action {
    let input_value = unwrap_action!(processor.pop_i8());
    let output_value = input_value as i16;

    unwrap_action!(processor.push_i16(output_value));

    Action::Ok
}

/// Pops a ?8 value from the stack, extends it to a i32 value.
///
/// Stack:
/// - ?8 - Input value.
/// + i32 - Output value.
pub fn extend_sign_8_to_32(processor: &mut Processor) -> Action {
    let input_value = unwrap_action!(processor.pop_i8());
    let output_value = input_value as i32;

    unwrap_action!(processor.push_i32(output_value));

    Action::Ok
}

/// Pops a ?16 value from the stack, extends it to a i32 value.
///
/// Stack:
/// - ?16 - Input value.
/// + i32 - Output value.
pub fn extend_sign_16_to_32(processor: &mut Processor) -> Action {
    let input_value = unwrap_action!(processor.pop_i16());
    let output_value = input_value as i32;

    unwrap_action!(processor.push_i32(output_value));

    Action::Ok
}

/// Pops a ?8 value from the stack, extends it to a i64 value.
///
/// Stack:
/// - ?8 - Input value.
/// + i64 - Output value.
pub fn extend_sign_8_to_64(processor: &mut Processor) -> Action {
    let input_value = unwrap_action!(processor.pop_i8());
    let output_value = input_value as i64;

    unwrap_action!(processor.push_i64(output_value));

    Action::Ok
}

/// Pops a ?16 value from the stack, extends it to a i64 value.
///
/// Stack:
/// - ?16 - Input value.
/// + i64 - Output value.
pub fn extend_sign_16_to_64(processor: &mut Processor) -> Action {
    let input_value = unwrap_action!(processor.pop_i16());
    let output_value = input_value as i64;

    unwrap_action!(processor.push_i64(output_value));

    Action::Ok
}

/// Pops a ?32 value from the stack, extends it to a i64 value.
///
/// Stack:
/// - ?32 - Input value.
/// + i64 - Output value.
pub fn extend_sign_32_to_64(processor: &mut Processor) -> Action {
    let input_value = unwrap_action!(processor.pop_i32());
    let output_value = input_value as i64;

    unwrap_action!(processor.push_i64(output_value));

    Action::Ok
}
