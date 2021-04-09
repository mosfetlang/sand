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

/// Expands the available memory of the system in a number of bytes.
///
/// Stack:
/// - u32 - Number of bytes.
/// + u32 - Previous size.
///
/// If it fails, the overflow_flag is set.
pub fn memory_grow(processor: &mut Processor) -> Result<(), Action> {
    let number_of_bytes = processor.pop_u32()?;
    let memory_size = processor.memory().size() as u32;
    let page_size = processor.memory().page_size() as u32;
    let mut pages = number_of_bytes / page_size;
    if number_of_bytes % page_size != 0 {
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

    let byte_size = std::mem::size_of::<u16>();
    for word in 0..number_of_words {
        memory.write_u16_at(start_pointer + word * byte_size, value)?;
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

    let byte_size = std::mem::size_of::<u32>();
    for word in 0..number_of_words {
        memory.write_u32_at(start_pointer + word * byte_size, value)?;
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

    let byte_size = std::mem::size_of::<u64>();
    for word in 0..number_of_words {
        memory.write_u64_at(start_pointer + word * byte_size, value)?;
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

    if target_pointer < origin_pointer {
        for i in 0..number_of_bytes {
            let value = memory.read_u8_at(origin_pointer + i)?;
            memory.write_u8_at(target_pointer + i, value)?;
        }
    } else {
        for i in (0..number_of_bytes).rev() {
            let value = memory.read_u8_at(origin_pointer + i)?;
            memory.write_u8_at(target_pointer + i, value)?;
        }
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

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::sasm::{Program, MEMORY_DEFAULT_PAGE_SIZE};

    use super::*;

    #[test]
    fn test_memory_size() {
        let program = Program::new_for_tests(Vec::new(), 0, 0);
        let mut processor = Processor::new_empty(program, 20);

        memory_size(&mut processor).expect("[1] The method must succeed");
        assert_eq!(
            processor.stack_pointer(),
            4,
            "[1] The stack pointer is incorrect"
        );
        assert_eq!(
            processor.pop_u32().unwrap() as usize,
            MEMORY_DEFAULT_PAGE_SIZE,
            "[1] The memory size is incorrect"
        );
    }

    #[test]
    fn test_memory_grow() {
        let program = Program::new_for_tests(Vec::new(), 0, 0);
        let mut processor = Processor::new_empty(program, 100);

        processor
            .push_u32(3 * MEMORY_DEFAULT_PAGE_SIZE as u32)
            .unwrap();
        assert_eq!(
            processor.stack_pointer(),
            4,
            "[1] The initial stack pointer is incorrect"
        );

        memory_grow(&mut processor).expect("[1] The method must succeed");
        assert_eq!(
            processor.stack_pointer(),
            4,
            "[1] The late stack pointer is incorrect"
        );
        assert_eq!(
            processor.pop_u32().unwrap() as usize,
            MEMORY_DEFAULT_PAGE_SIZE,
            "[1] The old memory size is incorrect"
        );
        assert_eq!(
            processor.memory().size(),
            4 * MEMORY_DEFAULT_PAGE_SIZE,
            "[1] The current memory size is incorrect"
        );
    }

    #[test]
    fn test_memory_fill() {
        let program = Program::new_for_tests(Vec::new(), 0, 0);
        let stack_size = 20;
        let mut processor = Processor::new_empty(program, 100);
        let max_bytes = 64 * 100;
        let start_pointer = stack_size;

        // Case 1
        let value = 0x45;
        let number_of_words = max_bytes / 8;
        processor.push_u32(start_pointer).unwrap();
        processor.push_u32(number_of_words).unwrap();
        processor.push_u8(value).unwrap();
        memory_fill_8(&mut processor).expect("[1] The method must succeed");
        assert_eq!(
            processor.stack_pointer(),
            0,
            "[1] The stack pointer is incorrect"
        );

        let page = &processor.memory().pages[0];
        let range = &page[start_pointer as usize..(start_pointer + number_of_words) as usize];
        for byte in range {
            assert_eq!(byte, &value, "[1] The value is incorrect")
        }

        // Case 2
        let value = 0x98ae;
        let number_of_words = max_bytes / 16;
        processor.push_u32(start_pointer).unwrap();
        processor.push_u32(number_of_words).unwrap();
        processor.push_u16(value).unwrap();
        memory_fill_16(&mut processor).expect("[2] The method must succeed");
        assert_eq!(
            processor.stack_pointer(),
            0,
            "[2] The stack pointer is incorrect"
        );

        let page = &processor.memory().pages[0];
        let range = &page[start_pointer as usize..(start_pointer + number_of_words) as usize];
        for (i, byte) in range.iter().enumerate() {
            match i % 2 {
                0 => assert_eq!(byte, &0xae, "[2.0] The value is incorrect"),
                1 => assert_eq!(byte, &0x98, "[2.1] The value is incorrect"),
                _ => unreachable!(),
            }
        }

        // Case 3
        let value = 0x12345678;
        let number_of_words = max_bytes / 16;
        processor.push_u32(start_pointer).unwrap();
        processor.push_u32(number_of_words).unwrap();
        processor.push_u32(value).unwrap();
        memory_fill_32(&mut processor).expect("[3] The method must succeed");
        assert_eq!(
            processor.stack_pointer(),
            0,
            "[3] The stack pointer is incorrect"
        );

        let page = &processor.memory().pages[0];
        let range = &page[start_pointer as usize..(start_pointer + number_of_words) as usize];
        for (i, byte) in range.iter().enumerate() {
            match i % 4 {
                0 => assert_eq!(byte, &0x78, "[3.0] The value is incorrect"),
                1 => assert_eq!(byte, &0x56, "[3.1] The value is incorrect"),
                2 => assert_eq!(byte, &0x34, "[3.2] The value is incorrect"),
                3 => assert_eq!(byte, &0x12, "[3.3] The value is incorrect"),
                _ => unreachable!(),
            }
        }

        // Case 4
        let value = 0x1234567890abcdef;
        let number_of_words = max_bytes / 16;
        processor.push_u32(start_pointer).unwrap();
        processor.push_u32(number_of_words).unwrap();
        processor.push_u64(value).unwrap();
        memory_fill_64(&mut processor).expect("[4] The method must succeed");
        assert_eq!(
            processor.stack_pointer(),
            0,
            "[4] The stack pointer is incorrect"
        );

        let page = &processor.memory().pages[0];
        let range = &page[start_pointer as usize..(start_pointer + number_of_words) as usize];
        for (i, byte) in range.iter().enumerate() {
            match i % 8 {
                0 => assert_eq!(byte, &0xef, "[4.0] The value is incorrect"),
                1 => assert_eq!(byte, &0xcd, "[4.1] The value is incorrect"),
                2 => assert_eq!(byte, &0xab, "[4.2] The value is incorrect"),
                3 => assert_eq!(byte, &0x90, "[4.3] The value is incorrect"),
                4 => assert_eq!(byte, &0x78, "[4.3] The value is incorrect"),
                5 => assert_eq!(byte, &0x56, "[4.3] The value is incorrect"),
                6 => assert_eq!(byte, &0x34, "[4.3] The value is incorrect"),
                7 => assert_eq!(byte, &0x12, "[4.3] The value is incorrect"),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_memory_copy() {
        let program = Program::new_for_tests(Vec::new(), 0, 0);
        let mut processor = Processor::new_empty(program, 100);

        let memory_index = 200;
        let target_memory_index = 400;
        let values = [0x10_u8, 0x25, 0x56, 0xe5];
        for (i, value) in values.iter().enumerate() {
            processor
                .memory_mut()
                .write_u8_at(memory_index + i, *value)
                .unwrap();
        }

        // Case 1: copy from a to a
        processor.push_u32(memory_index as u32).unwrap();
        processor.push_u32(values.len() as u32).unwrap();
        processor.push_u32(memory_index as u32).unwrap();
        memory_copy(&mut processor).expect("[1] The method must succeed");
        assert_eq!(
            processor.stack_pointer(),
            0,
            "[1] The stack pointer is incorrect"
        );

        for (i, value) in values.iter().enumerate() {
            let mem_value = processor.memory_mut().read_u8_at(memory_index + i).unwrap();

            assert_eq!(mem_value, *value, "[1.{}] The memory value is incorrect", i);
        }

        // Case 2: copy from a to b (not overlapping)
        processor.push_u32(memory_index as u32).unwrap();
        processor.push_u32(values.len() as u32).unwrap();
        processor.push_u32(target_memory_index).unwrap();
        memory_copy(&mut processor).expect("[2] The method must succeed");
        assert_eq!(
            processor.stack_pointer(),
            0,
            "[2] The stack pointer is incorrect"
        );

        for (i, value) in values.iter().enumerate() {
            let mem_value = processor.memory_mut().read_u8_at(memory_index + i).unwrap();

            assert_eq!(
                mem_value, *value,
                "[2.{}] The memory value at origin is incorrect",
                i
            );
        }

        for (i, value) in values.iter().enumerate() {
            let mem_value = processor
                .memory_mut()
                .read_u8_at(target_memory_index as usize + i)
                .unwrap();

            assert_eq!(
                mem_value, *value,
                "[2.{}] The memory value at target is incorrect",
                i
            );
        }

        // Case 3: copy from a to b (overlapping a < b)
        processor.push_u32(memory_index as u32).unwrap();
        processor.push_u32(values.len() as u32).unwrap();
        processor.push_u32(memory_index as u32 + 1).unwrap();
        memory_copy(&mut processor).expect("[3] The method must succeed");
        assert_eq!(
            processor.stack_pointer(),
            0,
            "[3] The stack pointer is incorrect"
        );

        {
            let mem_value = processor.memory_mut().read_u8_at(memory_index).unwrap();

            assert_eq!(
                mem_value, values[0],
                "[3.0] The memory value at origin is incorrect",
            );
        }

        for (i, value) in values.iter().enumerate() {
            let mem_value = processor
                .memory_mut()
                .read_u8_at(memory_index + i + 1)
                .unwrap();

            assert_eq!(
                mem_value, *value,
                "[3.{}] The memory value at target is incorrect",
                i
            );
        }

        // Case 4: copy from a to b (overlapping a > b)
        for (i, value) in values.iter().enumerate() {
            processor
                .memory_mut()
                .write_u8_at(memory_index + i, *value)
                .unwrap();
        }

        processor.push_u32(memory_index as u32).unwrap();
        processor.push_u32(values.len() as u32).unwrap();
        processor.push_u32(memory_index as u32 - 1).unwrap();
        memory_copy(&mut processor).expect("[4] The method must succeed");
        assert_eq!(
            processor.stack_pointer(),
            0,
            "[4] The stack pointer is incorrect"
        );

        {
            let mem_value = processor
                .memory_mut()
                .read_u8_at(memory_index + values.len() - 1)
                .unwrap();

            assert_eq!(
                mem_value,
                values[values.len() - 1],
                "[4.0] The memory value at origin is incorrect",
            );
        }

        for (i, value) in values.iter().enumerate() {
            let mem_value = processor
                .memory_mut()
                .read_u8_at(memory_index + i - 1)
                .unwrap();

            assert_eq!(
                mem_value, *value,
                "[4.{}] The memory value at target is incorrect",
                i
            );
        }
    }

    // TODO load
    // TODO Store
    // TODO data_load
}
