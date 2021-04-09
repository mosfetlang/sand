use crate::sasm::Action;

pub struct Program {
    program: Vec<u8>,
    data_pointer: usize,
    code_pointer: usize,
}

impl Program {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(program: Vec<u8>) -> Program {
        // TODO improve reading the format and the different sections.
        Program {
            program,
            data_pointer: 0,
            code_pointer: 0,
        }
    }

    #[cfg(test)]
    pub fn new_for_tests(program: Vec<u8>, data_pointer: usize, code_pointer: usize) -> Program {
        Program {
            program,
            data_pointer,
            code_pointer,
        }
    }

    // GETTERS ----------------------------------------------------------------

    #[inline]
    pub fn program(&self) -> &Vec<u8> {
        &self.program
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.program.len()
    }

    #[inline]
    pub fn data_pointer(&self) -> usize {
        self.data_pointer
    }

    #[inline]
    pub fn data_pointer_end(&self) -> usize {
        self.code_pointer
    }

    #[inline]
    pub fn code_pointer(&self) -> usize {
        self.code_pointer
    }

    #[inline]
    pub fn code_pointer_end(&self) -> usize {
        self.size()
    }

    // METHODS ----------------------------------------------------------------

    pub fn read_at(&self, index: usize, bytes: &mut [u8]) -> Result<(), Action> {
        let num_bytes = bytes.len();
        let last_index = index + num_bytes;
        if last_index > self.size() {
            return Err(Action::Panic("Segmentation Fault"));
        }

        bytes[..].clone_from_slice(&self.program[index..last_index]);

        Ok(())
    }

    #[inline]
    pub fn read_u8_at(&self, index: usize) -> Result<u8, Action> {
        let mut bytes = [0; std::mem::size_of::<u8>()];
        self.read_at(index, &mut bytes)
            .map(|_| u8::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_u16_at(&self, index: usize) -> Result<u16, Action> {
        let mut bytes = [0; std::mem::size_of::<u16>()];
        self.read_at(index, &mut bytes)
            .map(|_| u16::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_u32_at(&self, index: usize) -> Result<u32, Action> {
        let mut bytes = [0; std::mem::size_of::<u32>()];
        self.read_at(index, &mut bytes)
            .map(|_| u32::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_u64_at(&self, index: usize) -> Result<u64, Action> {
        let mut bytes = [0; std::mem::size_of::<u64>()];
        self.read_at(index, &mut bytes)
            .map(|_| u64::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_i8_at(&self, index: usize) -> Result<i8, Action> {
        let mut bytes = [0; std::mem::size_of::<i8>()];
        self.read_at(index, &mut bytes)
            .map(|_| i8::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_i16_at(&self, index: usize) -> Result<i16, Action> {
        let mut bytes = [0; std::mem::size_of::<i16>()];
        self.read_at(index, &mut bytes)
            .map(|_| i16::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_i32_at(&self, index: usize) -> Result<i32, Action> {
        let mut bytes = [0; std::mem::size_of::<i32>()];
        self.read_at(index, &mut bytes)
            .map(|_| i32::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_i64_at(&self, index: usize) -> Result<i64, Action> {
        let mut bytes = [0; std::mem::size_of::<i64>()];
        self.read_at(index, &mut bytes)
            .map(|_| i64::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_f32_at(&self, index: usize) -> Result<f32, Action> {
        let mut bytes = [0; std::mem::size_of::<f32>()];
        self.read_at(index, &mut bytes)
            .map(|_| f32::from_le_bytes(bytes))
    }

    #[inline]
    pub fn read_f64_at(&self, index: usize) -> Result<f64, Action> {
        let mut bytes = [0; std::mem::size_of::<f64>()];
        self.read_at(index, &mut bytes)
            .map(|_| f64::from_le_bytes(bytes))
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn test_memory_read_at() {
        let size = 5;
        let mut data = Vec::new();
        for i in 0..size {
            data.push(i + 1);
        }

        let program = Program::new(data);

        // Case 1: read one byte.
        let mut bytes = [0; 1];
        for i in 0..size {
            program
                .read_at(i as usize, &mut bytes)
                .expect(format!("[1] Cannot read byte at index {}", i).as_str());
            assert_eq!(bytes[0], i + 1, "The value is incorrect");
        }

        let result = program
            .read_at(5, &mut bytes)
            .expect_err("[1] The read must fail");
        assert_eq!(result.unwrap_panic(), "Segmentation Fault");

        // Case 2: read many bytes.
        let mut bytes = [0; 3];
        program
            .read_at(1, &mut bytes)
            .expect(format!("[2] Cannot read many bytes").as_str());
        assert_eq!(bytes[0], 2, "[2] The bytes[0] is incorrect");
        assert_eq!(bytes[1], 3, "[2] The bytes[1] is incorrect");
        assert_eq!(bytes[2], 4, "[2] The bytes[2] is incorrect");
    }

    #[test]
    fn test_memory() {
        let data = vec![0x12, 0x34, 0x56, 0x78, 0xef, 0xab, 0xcd, 0x09];
        let program = Program::new(data);

        // i8
        let value = 0x12;
        let result = program.read_i8_at(0).expect("[1] The read must succeed");
        assert_eq!(result, value, "[1] The value is incorrect");

        // i16
        let value = 0x3412;
        let result = program.read_i16_at(0).expect("[2] The read must succeed");
        assert_eq!(result, value, "[2] The value is incorrect");

        // i32
        let value = 0x78563412;
        let result = program.read_i32_at(0).expect("[3] The read must succeed");
        assert_eq!(result, value, "[3] The value is incorrect");

        // i64
        let value = 0x09cdabef78563412;
        let result = program.read_i64_at(0).expect("[4] The read must succeed");
        assert_eq!(result, value, "[4] The value is incorrect");

        // u8
        let value = 0x12;
        let result = program.read_u8_at(0).expect("[5] The read must succeed");
        assert_eq!(result, value, "[5] The value is incorrect");

        // u16
        let value = 0x3412;
        let result = program.read_u16_at(0).expect("[6] The read must succeed");
        assert_eq!(result, value, "[6] The value is incorrect");

        // u32
        let value = 0x78563412;
        let result = program.read_u32_at(0).expect("[7] The read must succeed");
        assert_eq!(result, value, "[7] The value is incorrect");

        // u64
        let value = 0x09cdabef78563412;
        let result = program.read_u64_at(0).expect("[8] The read must succeed");
        assert_eq!(result, value, "[8] The value is incorrect");

        // f32
        let value = f32::from_le_bytes(program.program[0..4].try_into().unwrap());
        let result = program.read_f32_at(0).expect("[9] The read must succeed");
        assert_eq!(result, value, "[9] The value is incorrect");

        // f64
        let value = f64::from_le_bytes(program.program[..].try_into().unwrap());
        let result = program.read_f64_at(0).expect("[10] The read must succeed");
        assert_eq!(result, value, "[10] The value is incorrect");
    }
}
