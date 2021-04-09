use std::convert::TryInto;

use crate::sasm::{Action, Processor};

/// Pops a ?8 value from the stack, extends it to a u16 value.
///
/// Stack:
/// - ?8 - Input value.
/// + u16 - Output value.
pub fn extend_8_to_16(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_u8()?;
    let output_value = input_value as u16;

    processor.push_u16(output_value)?;

    Ok(())
}

/// Pops a ?8 value from the stack, extends it to a u32 value.
///
/// Stack:
/// - ?8 - Input value.
/// + u32 - Output value.
pub fn extend_8_to_32(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_u8()?;
    let output_value = input_value as u32;

    processor.push_u32(output_value)?;

    Ok(())
}

/// Pops a ?16 value from the stack, extends it to a u32 value.
///
/// Stack:
/// - ?16 - Input value.
/// + u32 - Output value.
pub fn extend_16_to_32(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_u16()?;
    let output_value = input_value as u32;

    processor.push_u32(output_value)?;

    Ok(())
}

/// Pops a ?8 value from the stack, extends it to a u64 value.
///
/// Stack:
/// - ?8 - Input value.
/// + u64 - Output value.
pub fn extend_8_to_64(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_u8()?;
    let output_value = input_value as u64;

    processor.push_u64(output_value)?;

    Ok(())
}

/// Pops a ?16 value from the stack, extends it to a u64 value.
///
/// Stack:
/// - ?16 - Input value.
/// + u64 - Output value.
pub fn extend_16_to_64(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_u16()?;
    let output_value = input_value as u64;

    processor.push_u64(output_value)?;

    Ok(())
}

/// Pops a ?32 value from the stack, extends it to a u64 value.
///
/// Stack:
/// - ?32 - Input value.
/// + u64 - Output value.
pub fn extend_32_to_64(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_u32()?;
    let output_value = input_value as u64;

    processor.push_u64(output_value)?;

    Ok(())
}

/// Pops a ?8 value from the stack, extends it to a i16 value.
///
/// Stack:
/// - ?8 - Input value.
/// + i16 - Output value.
pub fn extend_sign_8_to_16(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_i8()?;
    let output_value = input_value as i16;

    processor.push_i16(output_value)?;

    Ok(())
}

/// Pops a ?8 value from the stack, extends it to a i32 value.
///
/// Stack:
/// - ?8 - Input value.
/// + i32 - Output value.
pub fn extend_sign_8_to_32(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_i8()?;
    let output_value = input_value as i32;

    processor.push_i32(output_value)?;

    Ok(())
}

/// Pops a ?16 value from the stack, extends it to a i32 value.
///
/// Stack:
/// - ?16 - Input value.
/// + i32 - Output value.
pub fn extend_sign_16_to_32(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_i16()?;
    let output_value = input_value as i32;

    processor.push_i32(output_value)?;

    Ok(())
}

/// Pops a ?8 value from the stack, extends it to a i64 value.
///
/// Stack:
/// - ?8 - Input value.
/// + i64 - Output value.
pub fn extend_sign_8_to_64(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_i8()?;
    let output_value = input_value as i64;

    processor.push_i64(output_value)?;

    Ok(())
}

/// Pops a ?16 value from the stack, extends it to a i64 value.
///
/// Stack:
/// - ?16 - Input value.
/// + i64 - Output value.
pub fn extend_sign_16_to_64(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_i16()?;
    let output_value = input_value as i64;

    processor.push_i64(output_value)?;

    Ok(())
}

/// Pops a ?32 value from the stack, extends it to a i64 value.
///
/// Stack:
/// - ?32 - Input value.
/// + i64 - Output value.
pub fn extend_sign_32_to_64(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_i32()?;
    let output_value = input_value as i64;

    processor.push_i64(output_value)?;

    Ok(())
}

/// Pops a ?16 value from the stack, truncates it to ?8 and pushes it to the stack.
///
/// Stack:
/// - ?16 - Input value.
/// + ?8  - Output value.
pub fn trunc_16_to_8(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_u16()?.to_le_bytes();
    let input_value = input_value.as_ref();
    let output_value = u8::from_le_bytes(input_value.try_into().unwrap());

    processor.push_u8(output_value)?;

    Ok(())
}

/// Pops a ?32 value from the stack, truncates it to ?8 and pushes it to the stack.
///
/// Stack:
/// - ?32 - Input value.
/// + ?8  - Output value.
pub fn trunc_32_to_8(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_u32()?.to_le_bytes();
    let input_value = input_value.as_ref();
    let output_value = u8::from_le_bytes(input_value.try_into().unwrap());

    processor.push_u8(output_value)?;

    Ok(())
}

/// Pops a ?32 value from the stack, truncates it to ?16 and pushes it to the stack.
///
/// Stack:
/// - ?32 - Input value.
/// + ?16  - Output value.
pub fn trunc_32_to_16(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_u32()?.to_le_bytes();
    let input_value = input_value.as_ref();
    let output_value = u16::from_le_bytes(input_value.try_into().unwrap());

    processor.push_u16(output_value)?;

    Ok(())
}

/// Pops a ?64 value from the stack, truncates it to ?8 and pushes it to the stack.
///
/// Stack:
/// - ?64 - Input value.
/// + ?8  - Output value.
pub fn trunc_64_to_8(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_u64()?.to_le_bytes();
    let input_value = input_value.as_ref();
    let output_value = u8::from_le_bytes(input_value.try_into().unwrap());

    processor.push_u8(output_value)?;

    Ok(())
}

/// Pops a ?64 value from the stack, truncates it to ?16 and pushes it to the stack.
///
/// Stack:
/// - ?64 - Input value.
/// + ?16  - Output value.
pub fn trunc_64_to_16(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_u64()?.to_le_bytes();
    let input_value = input_value.as_ref();
    let output_value = u16::from_le_bytes(input_value.try_into().unwrap());

    processor.push_u16(output_value)?;

    Ok(())
}

/// Pops a ?64 value from the stack, truncates it to ?32 and pushes it to the stack.
///
/// Stack:
/// - ?64 - Input value.
/// + ?32  - Output value.
pub fn trunc_64_to_32(processor: &mut Processor) -> Result<(), Action> {
    let input_value = processor.pop_u64()?.to_le_bytes();
    let input_value = input_value.as_ref();
    let output_value = u32::from_le_bytes(input_value.try_into().unwrap());

    processor.push_u32(output_value)?;

    Ok(())
}
