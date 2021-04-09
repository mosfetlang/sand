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

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::sasm::Program;

    use super::*;

    #[test]
    fn test_drop() {
        let program = Program::new_for_tests(Vec::new(), 0, 0);
        let mut processor = Processor::new_empty(program, 20);

        assert_eq!(
            processor.stack_pointer(),
            0,
            "[0.1] The stack pointer is incorrect"
        );
        processor.push_u64(0x9087654321abcdef).unwrap();
        processor.push_u64(0x9087654321abcdef).unwrap();
        assert_eq!(
            processor.stack_pointer(),
            16,
            "[0.2 The stack pointer is incorrect"
        );

        // Case 1
        drop_8(&mut processor).expect("[1] The drop must succeed");
        assert_eq!(
            processor.stack_pointer(),
            15,
            "[1] The stack pointer is incorrect"
        );

        // Case 2
        drop_16(&mut processor).expect("[2] The drop must succeed");
        assert_eq!(
            processor.stack_pointer(),
            13,
            "[2] The stack pointer is incorrect"
        );

        // Case 3
        drop_32(&mut processor).expect("[3] The drop must succeed");
        assert_eq!(
            processor.stack_pointer(),
            9,
            "[3] The stack pointer is incorrect"
        );

        // Case 4
        drop_64(&mut processor).expect("[4] The drop must succeed");
        assert_eq!(
            processor.stack_pointer(),
            1,
            "[4] The stack pointer is incorrect"
        );
    }

    #[test]
    fn test_const() {
        let program =
            Program::new_for_tests(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef], 0, 0);
        let mut processor = Processor::new_empty(program, 20);

        // Case 1
        const_8(&mut processor).expect("[1] The const must succeed");
        assert_eq!(
            processor.program_counter(),
            1,
            "[1] The stack pointer is incorrect"
        );
        assert_eq!(
            processor.stack_pointer(),
            1,
            "[1] The stack pointer is incorrect"
        );
        assert_eq!(
            processor.pop_u8().unwrap(),
            0x01,
            "[1] The value stored in the stack is incorrect"
        );

        processor.set_stack_pointer(0).unwrap();
        processor.set_program_counter(0).unwrap();

        // Case 2
        const_16(&mut processor).expect("[2] The const must succeed");
        assert_eq!(
            processor.program_counter(),
            2,
            "[2] The stack pointer is incorrect"
        );
        assert_eq!(
            processor.stack_pointer(),
            2,
            "[2] The stack pointer is incorrect"
        );
        assert_eq!(
            processor.pop_u16().unwrap(),
            0x2301,
            "[2] The value stored in the stack is incorrect"
        );

        processor.set_stack_pointer(0).unwrap();
        processor.set_program_counter(0).unwrap();

        // Case 3
        const_32(&mut processor).expect("[3] The const must succeed");
        assert_eq!(
            processor.program_counter(),
            4,
            "[3] The stack pointer is incorrect"
        );
        assert_eq!(
            processor.stack_pointer(),
            4,
            "[3] The stack pointer is incorrect"
        );
        assert_eq!(
            processor.pop_u32().unwrap(),
            0x67452301,
            "[3] The value stored in the stack is incorrect"
        );

        processor.set_stack_pointer(0).unwrap();
        processor.set_program_counter(0).unwrap();

        // Case 3
        const_64(&mut processor).expect("[4] The const must succeed");
        assert_eq!(
            processor.program_counter(),
            8,
            "[4] The stack pointer is incorrect"
        );
        assert_eq!(
            processor.stack_pointer(),
            8,
            "[4] The stack pointer is incorrect"
        );
        assert_eq!(
            processor.pop_u64().unwrap(),
            0xefcdab8967452301,
            "[4] The value stored in the stack is incorrect"
        );
    }
}
