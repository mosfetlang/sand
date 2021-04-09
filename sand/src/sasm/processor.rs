use crate::sasm::{Action, Memory, Program, MEMORY_DEFAULT_PAGE_SIZE, MEMORY_DEFAULT_STACK_SIZE};

/// A VM processor that carries with memory, registers, etc.
pub struct Processor {
    memory: Memory,
    program: Program,
    program_counter: usize,
    stack_pointer: usize,
    stack_size: usize,
    overflow_flag: bool,
}

impl Processor {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(memory: Memory, program: Program, stack_size: usize) -> Processor {
        assert!(
            stack_size <= memory.size(),
            "The stack size({}) must be lower or equal than the memory size({})",
            stack_size,
            memory.size()
        );

        Processor {
            memory,
            program,
            program_counter: 0,
            stack_pointer: 0,
            stack_size,
            overflow_flag: false,
        }
    }

    pub fn new_empty(program: Program, stack_size: usize) -> Processor {
        assert_eq!(
            stack_size % MEMORY_DEFAULT_PAGE_SIZE,
            0,
            "The stack size({}) must be a multiple of the page size({})",
            stack_size,
            MEMORY_DEFAULT_PAGE_SIZE
        );

        let mut memory = Memory::new_empty(MEMORY_DEFAULT_PAGE_SIZE, usize::MAX);
        memory
            .add_empty_pages(stack_size / MEMORY_DEFAULT_PAGE_SIZE)
            .unwrap();

        Processor {
            memory,
            program,
            program_counter: 0,
            stack_pointer: 0,
            stack_size,
            overflow_flag: false,
        }
    }

    // GETTERS ----------------------------------------------------------------

    #[inline]
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    #[inline]
    pub fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    #[inline]
    pub fn program(&self) -> &Program {
        &self.program
    }

    #[inline]
    pub fn program_counter(&self) -> usize {
        self.program_counter
    }

    #[inline]
    pub fn stack_pointer(&self) -> usize {
        self.stack_pointer
    }

    #[inline]
    pub fn stack_size(&self) -> usize {
        self.stack_pointer
    }

    #[inline]
    pub fn is_stack_empty(&self) -> bool {
        self.stack_pointer == 0
    }

    #[inline]
    pub fn is_stack_full(&self) -> bool {
        self.stack_pointer >= self.stack_size
    }

    #[inline]
    pub fn overflow_flag(&self) -> bool {
        self.overflow_flag
    }

    // SETTERS ----------------------------------------------------------------

    #[inline]
    pub fn set_program_counter(&mut self, program_counter: usize) {
        self.program_counter = program_counter;
    }

    /// # Safety
    ///
    /// This method will panic if the stack_pointer is outside the stack memory.
    pub fn set_stack_pointer(&mut self, stack_pointer: usize) {
        if stack_pointer >= self.stack_size {
            panic!("Stack overflow")
        }

        self.stack_pointer = stack_pointer;
    }

    #[inline]
    pub fn set_overflow_flag(&mut self, overflow_flag: bool) {
        self.overflow_flag = overflow_flag
    }

    // METHODS ----------------------------------------------------------------

    pub fn pop_u8(&mut self) -> Result<u8, Action> {
        let value = self.peek_u8()?;
        self.stack_pointer -= std::mem::size_of::<u8>();
        Ok(value)
    }

    pub fn pop_u16(&mut self) -> Result<u16, Action> {
        let value = self.peek_u16()?;
        self.stack_pointer -= std::mem::size_of::<u16>();
        Ok(value)
    }

    pub fn pop_u32(&mut self) -> Result<u32, Action> {
        let value = self.peek_u32()?;
        self.stack_pointer -= std::mem::size_of::<u32>();
        Ok(value)
    }

    pub fn pop_u64(&mut self) -> Result<u64, Action> {
        let value = self.peek_u64()?;
        self.stack_pointer -= std::mem::size_of::<u64>();
        Ok(value)
    }

    pub fn pop_i8(&mut self) -> Result<i8, Action> {
        let value = self.peek_i8()?;
        self.stack_pointer -= std::mem::size_of::<i8>();
        Ok(value)
    }

    pub fn pop_i16(&mut self) -> Result<i16, Action> {
        let value = self.peek_i16()?;
        self.stack_pointer -= std::mem::size_of::<i16>();
        Ok(value)
    }

    pub fn pop_i32(&mut self) -> Result<i32, Action> {
        let value = self.peek_i32()?;
        self.stack_pointer -= std::mem::size_of::<i32>();
        Ok(value)
    }

    pub fn pop_i64(&mut self) -> Result<i64, Action> {
        let value = self.peek_i64()?;
        self.stack_pointer -= std::mem::size_of::<i64>();
        Ok(value)
    }

    pub fn pop_f32(&mut self) -> Result<f32, Action> {
        let value = self.peek_f32()?;
        self.stack_pointer -= std::mem::size_of::<f32>();
        Ok(value)
    }

    pub fn pop_f64(&mut self) -> Result<f64, Action> {
        let value = self.peek_f64()?;
        self.stack_pointer -= std::mem::size_of::<f64>();
        Ok(value)
    }

    pub fn push_u8(&mut self, value: u8) -> Result<(), Action> {
        let num_bytes = std::mem::size_of::<u8>();
        if self.stack_pointer + num_bytes > self.stack_size {
            return Err(Action::Panic("Stack overflow"));
        }

        self.memory.write_u8_at(self.stack_pointer, value)?;
        self.stack_pointer += num_bytes;

        Ok(())
    }

    pub fn push_u16(&mut self, value: u16) -> Result<(), Action> {
        let num_bytes = std::mem::size_of::<u16>();
        if self.stack_pointer + num_bytes > self.stack_size {
            return Err(Action::Panic("Stack overflow"));
        }

        self.memory.write_u16_at(self.stack_pointer, value)?;
        self.stack_pointer += num_bytes;

        Ok(())
    }

    pub fn push_u32(&mut self, value: u32) -> Result<(), Action> {
        let num_bytes = std::mem::size_of::<u32>();
        if self.stack_pointer + num_bytes > self.stack_size {
            return Err(Action::Panic("Stack overflow"));
        }

        self.memory.write_u32_at(self.stack_pointer, value)?;
        self.stack_pointer += num_bytes;

        Ok(())
    }

    pub fn push_u64(&mut self, value: u64) -> Result<(), Action> {
        let num_bytes = std::mem::size_of::<u64>();
        if self.stack_pointer + num_bytes > self.stack_size {
            return Err(Action::Panic("Stack overflow"));
        }

        self.memory.write_u64_at(self.stack_pointer, value)?;
        self.stack_pointer += num_bytes;

        Ok(())
    }

    pub fn push_i8(&mut self, value: i8) -> Result<(), Action> {
        let num_bytes = std::mem::size_of::<i8>();
        if self.stack_pointer + num_bytes > self.stack_size {
            return Err(Action::Panic("Stack overflow"));
        }

        self.memory.write_i8_at(self.stack_pointer, value)?;
        self.stack_pointer += num_bytes;

        Ok(())
    }

    pub fn push_i16(&mut self, value: i16) -> Result<(), Action> {
        let num_bytes = std::mem::size_of::<i16>();
        if self.stack_pointer + num_bytes > self.stack_size {
            return Err(Action::Panic("Stack overflow"));
        }

        self.memory.write_i16_at(self.stack_pointer, value)?;
        self.stack_pointer += num_bytes;

        Ok(())
    }

    pub fn push_i32(&mut self, value: i32) -> Result<(), Action> {
        let num_bytes = std::mem::size_of::<i32>();
        if self.stack_pointer + num_bytes > self.stack_size {
            return Err(Action::Panic("Stack overflow"));
        }

        self.memory.write_i32_at(self.stack_pointer, value)?;
        self.stack_pointer += num_bytes;

        Ok(())
    }

    pub fn push_i64(&mut self, value: i64) -> Result<(), Action> {
        let num_bytes = std::mem::size_of::<i64>();
        if self.stack_pointer + num_bytes > self.stack_size {
            return Err(Action::Panic("Stack overflow"));
        }

        self.memory.write_i64_at(self.stack_pointer, value)?;
        self.stack_pointer += num_bytes;

        Ok(())
    }

    pub fn push_f32(&mut self, value: f32) -> Result<(), Action> {
        let num_bytes = std::mem::size_of::<f32>();
        if self.stack_pointer + num_bytes > self.stack_size {
            return Err(Action::Panic("Stack overflow"));
        }

        self.memory.write_f32_at(self.stack_pointer, value)?;
        self.stack_pointer += num_bytes;

        Ok(())
    }

    pub fn push_f64(&mut self, value: f64) -> Result<(), Action> {
        let num_bytes = std::mem::size_of::<f64>();
        if self.stack_pointer + num_bytes > self.stack_size {
            return Err(Action::Panic("Stack overflow"));
        }

        self.memory.write_f64_at(self.stack_pointer, value)?;
        self.stack_pointer += num_bytes;

        Ok(())
    }

    pub fn peek_u8(&self) -> Result<u8, Action> {
        let num_bytes = std::mem::size_of::<u8>();
        if num_bytes > self.stack_pointer {
            return Err(Action::Panic("Stack underflow"));
        }

        let start_index = self.stack_pointer - num_bytes;
        self.memory.read_u8_at(start_index)
    }

    pub fn peek_u16(&self) -> Result<u16, Action> {
        let num_bytes = std::mem::size_of::<u16>();
        if num_bytes > self.stack_pointer {
            return Err(Action::Panic("Stack underflow"));
        }

        let start_index = self.stack_pointer - num_bytes;
        self.memory.read_u16_at(start_index)
    }

    pub fn peek_u32(&self) -> Result<u32, Action> {
        let num_bytes = std::mem::size_of::<u32>();
        if num_bytes > self.stack_pointer {
            return Err(Action::Panic("Stack underflow"));
        }

        let start_index = self.stack_pointer - num_bytes;
        self.memory.read_u32_at(start_index)
    }

    pub fn peek_u64(&self) -> Result<u64, Action> {
        let num_bytes = std::mem::size_of::<u64>();
        if num_bytes > self.stack_pointer {
            return Err(Action::Panic("Stack underflow"));
        }

        let start_index = self.stack_pointer - num_bytes;
        self.memory.read_u64_at(start_index)
    }

    pub fn peek_i8(&self) -> Result<i8, Action> {
        let num_bytes = std::mem::size_of::<i8>();
        if num_bytes > self.stack_pointer {
            return Err(Action::Panic("Stack underflow"));
        }

        let start_index = self.stack_pointer - num_bytes;
        self.memory.read_i8_at(start_index)
    }

    pub fn peek_i16(&self) -> Result<i16, Action> {
        let num_bytes = std::mem::size_of::<i16>();
        if num_bytes > self.stack_pointer {
            return Err(Action::Panic("Stack underflow"));
        }

        let start_index = self.stack_pointer - num_bytes;
        self.memory.read_i16_at(start_index)
    }

    pub fn peek_i32(&self) -> Result<i32, Action> {
        let num_bytes = std::mem::size_of::<i32>();
        if num_bytes > self.stack_pointer {
            return Err(Action::Panic("Stack underflow"));
        }

        let start_index = self.stack_pointer - num_bytes;
        self.memory.read_i32_at(start_index)
    }

    pub fn peek_i64(&self) -> Result<i64, Action> {
        let num_bytes = std::mem::size_of::<i64>();
        if num_bytes > self.stack_pointer {
            return Err(Action::Panic("Stack underflow"));
        }

        let start_index = self.stack_pointer - num_bytes;
        self.memory.read_i64_at(start_index)
    }

    pub fn peek_f32(&self) -> Result<f32, Action> {
        let num_bytes = std::mem::size_of::<f32>();
        if num_bytes > self.stack_pointer {
            return Err(Action::Panic("Stack underflow"));
        }

        let start_index = self.stack_pointer - num_bytes;
        self.memory.read_f32_at(start_index)
    }

    pub fn peek_f64(&self) -> Result<f64, Action> {
        let num_bytes = std::mem::size_of::<f64>();
        if num_bytes > self.stack_pointer {
            return Err(Action::Panic("Stack underflow"));
        }

        let start_index = self.stack_pointer - num_bytes;
        self.memory.read_f64_at(start_index)
    }

    pub fn code_next_u8(&mut self) -> Result<u8, Action> {
        let result = self.program.read_u8_at(self.program_counter)?;
        self.program_counter += std::mem::size_of::<u8>();
        Ok(result)
    }

    pub fn code_next_u16(&mut self) -> Result<u16, Action> {
        let result = self.program.read_u16_at(self.program_counter)?;
        self.program_counter += std::mem::size_of::<u16>();
        Ok(result)
    }

    pub fn code_next_u32(&mut self) -> Result<u32, Action> {
        let result = self.program.read_u32_at(self.program_counter)?;
        self.program_counter += std::mem::size_of::<u32>();
        Ok(result)
    }

    pub fn code_next_u64(&mut self) -> Result<u64, Action> {
        let result = self.program.read_u64_at(self.program_counter)?;
        self.program_counter += std::mem::size_of::<u64>();
        Ok(result)
    }

    pub fn code_next_i8(&mut self) -> Result<i8, Action> {
        let result = self.program.read_i8_at(self.program_counter)?;
        self.program_counter += std::mem::size_of::<i8>();
        Ok(result)
    }

    pub fn code_next_i16(&mut self) -> Result<i16, Action> {
        let result = self.program.read_i16_at(self.program_counter)?;
        self.program_counter += std::mem::size_of::<i16>();
        Ok(result)
    }

    pub fn code_next_i32(&mut self) -> Result<i32, Action> {
        let result = self.program.read_i32_at(self.program_counter)?;
        self.program_counter += std::mem::size_of::<i32>();
        Ok(result)
    }

    pub fn code_next_i64(&mut self) -> Result<i64, Action> {
        let result = self.program.read_i64_at(self.program_counter)?;
        self.program_counter += std::mem::size_of::<i64>();
        Ok(result)
    }

    pub fn code_next_f32(&mut self) -> Result<f32, Action> {
        let result = self.program.read_f32_at(self.program_counter)?;
        self.program_counter += std::mem::size_of::<f32>();
        Ok(result)
    }

    pub fn code_next_f64(&mut self) -> Result<f64, Action> {
        let result = self.program.read_f64_at(self.program_counter)?;
        self.program_counter += std::mem::size_of::<f64>();
        Ok(result)
    }
}
