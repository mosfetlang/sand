use crate::sasm::{Action, Processor};

/// Drops a ?8 from the stack.
///
/// Stack:
/// - ?8
pub fn drop_8(processor: &mut Processor) -> Result<(), Action> {
    processor.pop_u8()?;
    Ok(())
}

/// Drops a ?16 from the stack.
///
/// Stack:
/// - ?16
pub fn drop_16(processor: &mut Processor) -> Result<(), Action> {
    processor.pop_u16()?;
    Ok(())
}

/// Drops a ?32 from the stack.
///
/// Stack:
/// - ?32
pub fn drop_32(processor: &mut Processor) -> Result<(), Action> {
    processor.pop_u32()?;
    Ok(())
}

/// Drops a ?64 from the stack.
///
/// Stack:
/// - ?64
pub fn drop_64(processor: &mut Processor) -> Result<(), Action> {
    processor.pop_u64()?;
    Ok(())
}

/// Add a ?8 const to the stack.
///
/// Stack:
/// + ?8
pub fn const_8(processor: &mut Processor) -> Result<(), Action> {
    let result = processor.code_next_u8()?;
    processor.push_u8(result)?;
    Ok(())
}

/// Add a ?16 const to the stack.
///
/// Stack:
/// + ?16
pub fn const_16(processor: &mut Processor) -> Result<(), Action> {
    let result = processor.code_next_u16()?;
    processor.push_u16(result)?;
    Ok(())
}

/// Add a ?32 const to the stack.
///
/// Stack:
/// + ?32
pub fn const_32(processor: &mut Processor) -> Result<(), Action> {
    let result = processor.code_next_u32()?;
    processor.push_u32(result)?;
    Ok(())
}

/// Add a ?64 const to the stack.
///
/// Stack:
/// + ?64
pub fn const_64(processor: &mut Processor) -> Result<(), Action> {
    let result = processor.code_next_u64()?;
    processor.push_u64(result)?;
    Ok(())
}
