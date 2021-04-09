use num_integer::Integer;

use crate::sasm::Action;

/// The default memory page size: 64KiB
pub const MEMORY_DEFAULT_PAGE_SIZE: usize = 64 * 1024;

/// The default stack size: 2MiB
pub const MEMORY_DEFAULT_STACK_SIZE: usize = 2 * 1024 * 1024;

/// A paginated memory abstraction.
pub struct Memory {
    page_size: usize,
    max_pages: usize,
    pub pages: Vec<Vec<u8>>,
}

impl Memory {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(page_size: usize, max_pages: usize, pages: Vec<Vec<u8>>) -> Memory {
        assert_ne!(page_size, 0, "The page size cannot be zero");

        let page_count = pages.len();
        assert!(
            page_count <= max_pages,
            "The number of heap pages({}) is greater than the maximum({})",
            page_count,
            max_pages
        );

        for (i, page) in pages.iter().enumerate() {
            let current_page_size = page.len();
            assert_eq!(
                current_page_size, page_size,
                "The page[{}] size({}) is not equal to the page size({})",
                i, current_page_size, page_size
            );
        }

        Memory {
            page_size,
            max_pages,
            pages,
        }
    }

    pub fn new_empty(page_size: usize, max_pages: usize) -> Memory {
        assert_ne!(page_size, 0, "The page size cannot be zero");

        Memory {
            page_size,
            max_pages,
            pages: Vec::new(),
        }
    }

    // GETTERS ----------------------------------------------------------------

    #[inline]
    pub fn page_size(&self) -> usize {
        self.page_size
    }

    #[inline]
    pub fn max_pages(&self) -> usize {
        self.max_pages
    }

    #[inline]
    pub fn pages(&self) -> usize {
        self.pages.len()
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.pages.len() * self.page_size
    }

    // METHODS ----------------------------------------------------------------

    pub fn read_at(&self, index: usize, bytes: &mut [u8]) -> Result<(), Action> {
        let num_bytes = bytes.len();
        if index + num_bytes > self.size() {
            return Err(Action::Panic("Segmentation Fault"));
        }

        let (mut page_index, mut index_in_page) = index.div_rem(&self.page_size);
        let mut index_in_bytes = 0;
        loop {
            let num_bytes = num_bytes - index_in_bytes;
            let last_index_in_page = index_in_page + num_bytes;
            if last_index_in_page > self.page_size {
                // Value in multiple pages.
                let last_index_in_bytes = index_in_bytes + (self.page_size - index_in_page);
                self.read_at_single_page(
                    page_index,
                    index_in_page,
                    &mut bytes[index_in_bytes..last_index_in_bytes],
                );

                page_index += 1;
                index_in_page = 0;
                index_in_bytes = last_index_in_bytes;
            } else {
                // Value in single page.
                self.read_at_single_page(page_index, index_in_page, &mut bytes[index_in_bytes..]);
                break;
            }
        }

        Ok(())
    }

    fn read_at_single_page(&self, page_index: usize, index_in_page: usize, bytes: &mut [u8]) {
        let page = &self.pages[page_index];
        let bytes_range = ..(bytes.len() - index_in_page);
        let page_range = index_in_page..bytes.len();
        bytes[bytes_range].clone_from_slice(&page[page_range]);
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

    pub fn write_at(&mut self, index: usize, bytes: &[u8]) -> Result<(), Action> {
        let num_bytes = bytes.len();
        if index + num_bytes > self.size() {
            return Err(Action::Panic("Segmentation Fault"));
        }

        let (mut page_index, mut index_in_page) = index.div_rem(&self.page_size);
        let mut index_in_bytes = 0;
        loop {
            let num_bytes = num_bytes - index_in_bytes;
            let last_index_in_page = index_in_page + num_bytes;
            if last_index_in_page > self.page_size {
                // Value in multiple pages.
                let last_index_in_bytes = index_in_bytes + (self.page_size - index_in_page);
                self.write_at_single_page(
                    page_index,
                    index_in_page,
                    &bytes[index_in_bytes..last_index_in_bytes],
                );

                page_index += 1;
                index_in_page = 0;
                index_in_bytes = last_index_in_bytes;
            } else {
                // Value in single page.
                self.write_at_single_page(page_index, index_in_page, &bytes[index_in_bytes..]);
                break;
            }
        }

        Ok(())
    }

    fn write_at_single_page(&mut self, page_index: usize, index_in_page: usize, bytes: &[u8]) {
        let page = &mut self.pages[page_index];
        page[index_in_page..bytes.len()].clone_from_slice(&bytes[..(bytes.len() - index_in_page)]);
    }

    #[inline]
    pub fn write_u8_at(&mut self, index: usize, value: u8) -> Result<(), Action> {
        self.write_at(index, &value.to_le_bytes())
    }

    #[inline]
    pub fn write_u16_at(&mut self, index: usize, value: u16) -> Result<(), Action> {
        self.write_at(index, &value.to_le_bytes())
    }

    #[inline]
    pub fn write_u32_at(&mut self, index: usize, value: u32) -> Result<(), Action> {
        self.write_at(index, &value.to_le_bytes())
    }

    #[inline]
    pub fn write_u64_at(&mut self, index: usize, value: u64) -> Result<(), Action> {
        self.write_at(index, &value.to_le_bytes())
    }

    #[inline]
    pub fn write_i8_at(&mut self, index: usize, value: i8) -> Result<(), Action> {
        self.write_at(index, &value.to_le_bytes())
    }

    #[inline]
    pub fn write_i16_at(&mut self, index: usize, value: i16) -> Result<(), Action> {
        self.write_at(index, &value.to_le_bytes())
    }

    #[inline]
    pub fn write_i32_at(&mut self, index: usize, value: i32) -> Result<(), Action> {
        self.write_at(index, &value.to_le_bytes())
    }

    #[inline]
    pub fn write_i64_at(&mut self, index: usize, value: i64) -> Result<(), Action> {
        self.write_at(index, &value.to_le_bytes())
    }

    #[inline]
    pub fn write_f32_at(&mut self, index: usize, value: f32) -> Result<(), Action> {
        self.write_at(index, &value.to_le_bytes())
    }

    #[inline]
    pub fn write_f64_at(&mut self, index: usize, value: f64) -> Result<(), Action> {
        self.write_at(index, &value.to_le_bytes())
    }

    #[inline]
    pub fn add_page(&mut self, page: Vec<u8>) -> Result<(), Action> {
        assert_eq!(
            page.len(),
            self.page_size,
            "Cannot insert a page whose length is {} into a memory with page size({})",
            page.len(),
            self.page_size
        );

        let new_pages = self.pages() + 1;
        if new_pages > self.max_pages {
            return Err(Action::Panic("Memory out of bounds"));
        }

        self.pages.push(page);

        Ok(())
    }

    #[inline]
    pub fn add_empty_page(&mut self) -> Result<(), Action> {
        self.add_empty_pages(1)
    }

    pub fn add_empty_pages(&mut self, amount: usize) -> Result<(), Action> {
        let new_pages = self.pages() + amount;
        if new_pages > self.max_pages {
            return Err(Action::Panic("Memory out of bounds"));
        }

        let page_size = self.page_size;
        self.pages.resize_with(new_pages, || {
            let mut page = Vec::with_capacity(page_size);
            unsafe { page.set_len(page_size) }
            page
        });

        Ok(())
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory::new_empty(MEMORY_DEFAULT_PAGE_SIZE, usize::MAX)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_memory_grow() {
        let mut memory = Memory::new_empty(1, 5);
        memory
            .add_empty_page()
            .expect("[1] Cannot add an empty page");
        memory
            .add_empty_pages(3)
            .expect("[1] Cannot add many empty pages");
        memory
            .add_page(vec![2])
            .expect("[1] Cannot add a custom page");

        let result = memory
            .add_empty_page()
            .expect_err("[1] The addition of another empty page must fail");
        assert_eq!(result.unwrap_panic(), "Memory out of bounds");

        let result = memory
            .add_empty_pages(5)
            .expect_err("[1] The addition of other empty pages must fail");
        assert_eq!(result.unwrap_panic(), "Memory out of bounds");

        let result = memory
            .add_empty_page()
            .expect_err("[1] The addition of another custom page must fail");
        assert_eq!(result.unwrap_panic(), "Memory out of bounds");
    }

    #[test]
    fn test_memory_read_at() {
        let max_pages = 5;
        let mut memory = Memory::new_empty(1, max_pages);
        for i in 1..max_pages + 1 {
            memory
                .add_page(vec![i as u8])
                .expect(format!("[1] Cannot add a custom page for {}", i).as_str());
        }

        // Case 1: read one byte.
        let mut bytes = [0; 1];
        for i in 0..max_pages {
            memory
                .read_at(i, &mut bytes)
                .expect(format!("[1] Cannot read byte at index {}", i).as_str());
            assert_eq!(bytes[0], (i + 1) as u8, "The value is incorrect");
        }

        let result = memory
            .read_at(5, &mut bytes)
            .expect_err("[1] The read must fail");
        assert_eq!(result.unwrap_panic(), "Segmentation Fault");

        // Case 2: read many bytes.
        let mut bytes = [0; 3];
        memory
            .read_at(1, &mut bytes)
            .expect(format!("[2] Cannot read many bytes").as_str());
        assert_eq!(bytes[0], 2, "[2] The bytes[0] is incorrect");
        assert_eq!(bytes[1], 3, "[2] The bytes[1] is incorrect");
        assert_eq!(bytes[2], 4, "[2] The bytes[2] is incorrect");
    }

    #[test]
    fn test_memory_write_at() {
        // Case 1: write one byte.
        let max_pages = 5;
        let mut memory = Memory::new_empty(1, max_pages);
        for i in 1..max_pages + 1 {
            memory
                .add_page(vec![i as u8])
                .expect(format!("[1] Cannot add a custom page for {}", i).as_str());
        }

        for i in 0..max_pages {
            let mut bytes = [0; 1];
            bytes[0] = (i + 1) as u8;
            memory
                .write_at(i, &mut bytes)
                .expect(format!("[1] Cannot write byte at index {}", i).as_str());
            assert_eq!(memory.pages[i][0], (i + 1) as u8, "The value is incorrect");
        }

        let mut bytes = [5; 1];
        let result = memory
            .write_at(5, &mut bytes)
            .expect_err("[1] The write must fail");
        assert_eq!(result.unwrap_panic(), "Segmentation Fault");

        // Case 2: write many bytes.
        let mut memory = Memory::new_empty(1, max_pages);
        for i in 1..max_pages + 1 {
            memory
                .add_page(vec![i as u8])
                .expect(format!("[2] Cannot add a custom page for {}", i).as_str());
        }

        let mut bytes = [0; 3];
        memory
            .write_at(1, &mut bytes)
            .expect(format!("[2] Cannot read many bytes").as_str());
        assert_eq!(memory.pages[0][0], 1, "[2] The value[0] is incorrect");
        assert_eq!(memory.pages[1][0], 0, "[2] The value[1] is incorrect");
        assert_eq!(memory.pages[2][0], 0, "[2] The value[2] is incorrect");
        assert_eq!(memory.pages[3][0], 0, "[2] The value[3] is incorrect");
        assert_eq!(memory.pages[4][0], 5, "[2] The value[4] is incorrect");
    }

    #[test]
    fn test_memory() {
        let mut memory = Memory::new_empty(20, 1);
        memory.add_empty_page().expect("Cannot add an empty page");

        // i8
        let value = 0x1f;
        memory
            .write_i8_at(0, value)
            .expect("[1] The write must succeed");
        let result = memory.read_i8_at(0).expect("[1] The read must succeed");
        assert_eq!(result, value, "[1] The value is incorrect");

        // i16
        let value = 0x12ef;
        memory
            .write_i16_at(0, value)
            .expect("[2] The write must succeed");
        let result = memory.read_i16_at(0).expect("[2] The read must succeed");
        assert_eq!(result, value, "[2] The value is incorrect");

        // i32
        let value = 0x1234cdef;
        memory
            .write_i32_at(0, value)
            .expect("[3] The write must succeed");
        let result = memory.read_i32_at(0).expect("[3] The read must succeed");
        assert_eq!(result, value, "[3] The value is incorrect");

        // i64
        let value = 0x1234567890abcdef;
        memory
            .write_i64_at(0, value)
            .expect("[4] The write must succeed");
        let result = memory.read_i64_at(0).expect("[4] The read must succeed");
        assert_eq!(result, value, "[4] The value is incorrect");

        // u8
        let value = 0x1f;
        memory
            .write_u8_at(0, value)
            .expect("[5] The write must succeed");
        let result = memory.read_u8_at(0).expect("[5] The read must succeed");
        assert_eq!(result, value, "[5] The value is incorrect");

        // u16
        let value = 0x12ef;
        memory
            .write_u16_at(0, value)
            .expect("[6] The write must succeed");
        let result = memory.read_u16_at(0).expect("[6] The read must succeed");
        assert_eq!(result, value, "[6] The value is incorrect");

        // u32
        let value = 0x1234cdef;
        memory
            .write_u32_at(0, value)
            .expect("[7] The write must succeed");
        let result = memory.read_u32_at(0).expect("[7] The read must succeed");
        assert_eq!(result, value, "[7] The value is incorrect");

        // u64
        let value = 0x1234567890abcdef;
        memory
            .write_u64_at(0, value)
            .expect("[8] The write must succeed");
        let result = memory.read_u64_at(0).expect("[8] The read must succeed");
        assert_eq!(result, value, "[8] The value is incorrect");

        // f32
        let value = 123456789.987654321;
        memory
            .write_f32_at(0, value)
            .expect("[9] The write must succeed");
        let result = memory.read_f32_at(0).expect("[9] The read must succeed");
        assert_eq!(result, value, "[9] The value is incorrect");

        // f64
        let value = 123456789.987654321;
        memory
            .write_f64_at(0, value)
            .expect("[10] The write must succeed");
        let result = memory.read_f64_at(0).expect("[10] The read must succeed");
        assert_eq!(result, value, "[10] The value is incorrect");
    }
}
