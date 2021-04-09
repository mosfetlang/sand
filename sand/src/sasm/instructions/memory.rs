use crate::sasm::{Action, Processor};

/// Push the current memory size in bytes to the stack.
///
/// Stack:
/// + u32 - Memory size
pub fn memory_size(processor: &mut Processor) -> Result<(), Action> {
    let memory_size = processor.memory().size() as u32;
    processor.push_u32(memory_size)?;
    Ok(())
}

/// Expands the available memory of the system.
///
/// Stack:
/// - u32 - Increase amount
/// + u32 - Previous size.
///
/// If it fails, the overflow_flag is set.
pub fn memory_grow(processor: &mut Processor) -> Result<(), Action> {
    let increase_amount = processor.pop_u32()?;
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
    processor.push_u32(memory_size)?;

    Ok(())
}

/// Fills a region of the memory with the specified value of ?8.
/// Can cause a panic when the memory region is unavailable.
///
/// Stack:
/// - u8  - The value to use to fill the memory.
/// - u32 - Number of words.
/// - u32 - Start pointer.
pub fn memory_fill_8(processor: &mut Processor) -> Result<(), Action> {
    let value = processor.pop_u8()?;
    let number_of_words = processor.pop_u32()? as usize;
    let start_pointer = processor.pop_u32()? as usize;

    let memory = processor.memory_mut();

    for word in 0..number_of_words {
        memory.write_u8_at(start_pointer + word, value)?;
    }

    Ok(())
}

/// Fills a region of the memory with the specified value of ?16.
/// Can cause a panic when the memory region is unavailable.
///
/// Stack:
/// - u16 - The value to use to fill the memory.
/// - u32 - Number of words.
/// - u32 - Start pointer.
pub fn memory_fill_16(processor: &mut Processor) -> Result<(), Action> {
    let value = processor.pop_u16()?;
    let number_of_words = processor.pop_u32()? as usize;
    let start_pointer = processor.pop_u32()? as usize;

    let memory = processor.memory_mut();

    for word in 0..number_of_words {
        memory.write_u16_at(start_pointer + word, value)?;
    }

    Ok(())
}

/// Fills a region of the memory with the specified value of ?32.
/// Can cause a panic when the memory region is unavailable.
///
/// Stack:
/// - u32 - The value to use to fill the memory.
/// - u32 - Number of words.
/// - u32 - Start pointer.
pub fn memory_fill_32(processor: &mut Processor) -> Result<(), Action> {
    let value = processor.pop_u32()?;
    let number_of_words = processor.pop_u32()? as usize;
    let start_pointer = processor.pop_u32()? as usize;

    let memory = processor.memory_mut();

    for word in 0..number_of_words {
        memory.write_u32_at(start_pointer + word, value)?;
    }

    Ok(())
}

/// Fills a region of the memory with the specified value of ?64.
/// Can cause a panic when the memory region is unavailable.
///
/// Stack:
/// - u64 - The value to use to fill the memory.
/// - u32 - Number of words.
/// - u32 - Start pointer.
pub fn memory_fill_64(processor: &mut Processor) -> Result<(), Action> {
    let value = processor.pop_u64()?;
    let number_of_words = processor.pop_u32()? as usize;
    let start_pointer = processor.pop_u32()? as usize;

    let memory = processor.memory_mut();

    for word in 0..number_of_words {
        memory.write_u64_at(start_pointer + word, value)?;
    }

    Ok(())
}

/// Copies a region of memory into another one.
/// Can cause a panic when either the origin or target memory regions are unavailable.
///
/// Stack:
/// - u32 - Target pointer.
/// - u32 - Number of bytes.
/// - u32 - Origin pointer.
pub fn memory_copy(processor: &mut Processor) -> Result<(), Action> {
    let target_pointer = processor.pop_u32()? as usize;
    let number_of_bytes = processor.pop_u32()? as usize;
    let origin_pointer = processor.pop_u32()? as usize;

    if origin_pointer == target_pointer {
        return Ok(());
    }

    let memory = processor.memory_mut();

    for i in 0..number_of_bytes {
        let value = memory.read_u8_at(origin_pointer + i)?;
        memory.write_u8_at(target_pointer + i, value)?;
    }

    Ok(())
}

/// Loads a ?8 memory value and pushes it into the stack.
/// Can cause a panic when the memory position is unavailable.
///
/// Stack:
/// - u32 - Memory position.
/// + ?8  - Memory value.
pub fn memory_load_8(processor: &mut Processor) -> Result<(), Action> {
    let memory_position = processor.pop_u32()? as usize;

    let memory = processor.memory();
    let value = memory.read_u8_at(memory_position)?;
    processor.push_u8(value)?;

    Ok(())
}

/// Loads a ?16 memory value and pushes it into the stack.
/// Can cause a panic when the memory position is unavailable.
///
/// Stack:
/// - u32 - Memory position.
/// + ?16 - Memory value.
pub fn memory_load_16(processor: &mut Processor) -> Result<(), Action> {
    let memory_position = processor.pop_u32()? as usize;

    let memory = processor.memory();
    let value = memory.read_u16_at(memory_position)?;
    processor.push_u16(value)?;

    Ok(())
}

/// Loads a ?32 memory value and pushes it into the stack.
/// Can cause a panic when the memory position is unavailable.
///
/// Stack:
/// - u32 - Memory position.
/// + ?32 - Memory value.
pub fn memory_load_32(processor: &mut Processor) -> Result<(), Action> {
    let memory_position = processor.pop_u32()? as usize;

    let memory = processor.memory();
    let value = memory.read_u32_at(memory_position)?;
    processor.push_u32(value)?;

    Ok(())
}

/// Loads a ?64 memory value and pushes it into the stack.
/// Can cause a panic when the memory position is unavailable.
///
/// Stack:
/// - u32 - Memory position.
/// + ?64 - Memory value.
pub fn memory_load_64(processor: &mut Processor) -> Result<(), Action> {
    let memory_position = processor.pop_u32()? as usize;

    let memory = processor.memory();
    let value = memory.read_u64_at(memory_position)?;
    processor.push_u64(value)?;

    Ok(())
}

/// Pops a ?8 value from the stack and stores it in memory.
/// Can cause a panic when the memory position is unavailable.
///
/// Stack:
/// - ?8  - Value.
/// - u32 - Memory position.
pub fn memory_store_8(processor: &mut Processor) -> Result<(), Action> {
    let value = processor.pop_u8()?;
    let memory_position = processor.pop_u32()? as usize;

    let memory = processor.memory_mut();
    memory.write_u8_at(memory_position, value)?;

    Ok(())
}

/// Pops a ?16 value from the stack and stores it in memory.
/// Can cause a panic when the memory position is unavailable.
///
/// Stack:
/// - ?16  - Value.
/// - u32 - Memory position.
pub fn memory_store_16(processor: &mut Processor) -> Result<(), Action> {
    let value = processor.pop_u16()?;
    let memory_position = processor.pop_u32()? as usize;

    let memory = processor.memory_mut();
    memory.write_u16_at(memory_position, value)?;

    Ok(())
}

/// Pops a ?32 value from the stack and stores it in memory.
/// Can cause a panic when the memory position is unavailable.
///
/// Stack:
/// - ?32  - Value.
/// - u32 - Memory position.
pub fn memory_store_32(processor: &mut Processor) -> Result<(), Action> {
    let value = processor.pop_u32()?;
    let memory_position = processor.pop_u32()? as usize;

    let memory = processor.memory_mut();
    memory.write_u32_at(memory_position, value)?;

    Ok(())
}

/// Pops a ?64 value from the stack and stores it in memory.
/// Can cause a panic when the memory position is unavailable.
///
/// Stack:
/// - ?64  - Value.
/// - u32 - Memory position.
pub fn memory_store_64(processor: &mut Processor) -> Result<(), Action> {
    let value = processor.pop_u64()?;
    let memory_position = processor.pop_u32()? as usize;

    let memory = processor.memory_mut();
    memory.write_u64_at(memory_position, value)?;

    Ok(())
}

/// Loads a ?8 value from the program data and pushes it into the stack.
/// Can cause a panic when the memory position is unavailable.
///
/// Stack:
/// - u32 - Memory position.
/// + ?8  - Program data value.
pub fn program_data_load_8(processor: &mut Processor) -> Result<(), Action> {
    let memory_position = processor.pop_u32()? as usize;

    let program = processor.program();

    let last_position = memory_position + std::mem::size_of::<u8>();
    if memory_position < program.data_pointer() || last_position > program.data_pointer_end() {
        return Err(Action::Panic("Data Segmentation Fault"));
    }

    let value = program.read_u8_at(memory_position)?;
    processor.push_u8(value)?;

    Ok(())
}

/// Loads a ?16 value from the program data and pushes it into the stack.
/// Can cause a panic when the memory position is unavailable.
///
/// Stack:
/// - u32 - Memory position.
/// + ?16  - Program data value.
pub fn program_data_load_16(processor: &mut Processor) -> Result<(), Action> {
    let memory_position = processor.pop_u32()? as usize;

    let program = processor.program();

    let last_position = memory_position + std::mem::size_of::<u16>();
    if memory_position < program.data_pointer() || last_position > program.data_pointer_end() {
        return Err(Action::Panic("Data Segmentation Fault"));
    }

    let value = program.read_u16_at(memory_position)?;
    processor.push_u16(value)?;

    Ok(())
}

/// Loads a ?32 value from the program data and pushes it into the stack.
/// Can cause a panic when the memory position is unavailable.
///
/// Stack:
/// - u32 - Memory position.
/// + ?32  - Program data value.
pub fn program_data_load_32(processor: &mut Processor) -> Result<(), Action> {
    let memory_position = processor.pop_u32()? as usize;

    let program = processor.program();

    let last_position = memory_position + std::mem::size_of::<u32>();
    if memory_position < program.data_pointer() || last_position > program.data_pointer_end() {
        return Err(Action::Panic("Data Segmentation Fault"));
    }

    let value = program.read_u32_at(memory_position)?;
    processor.push_u32(value)?;

    Ok(())
}

/// Loads a ?64 value from the program data and pushes it into the stack.
/// Can cause a panic when the memory position is unavailable.
///
/// Stack:
/// - u32 - Memory position.
/// + ?64  - Program data value.
pub fn program_data_load_64(processor: &mut Processor) -> Result<(), Action> {
    let memory_position = processor.pop_u32()? as usize;

    let program = processor.program();

    let last_position = memory_position + std::mem::size_of::<u64>();
    if memory_position < program.data_pointer() || last_position > program.data_pointer_end() {
        return Err(Action::Panic("Data Segmentation Fault"));
    }

    let value = program.read_u64_at(memory_position)?;
    processor.push_u64(value)?;

    Ok(())
}
