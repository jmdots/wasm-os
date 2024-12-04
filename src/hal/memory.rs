/// Hardware Abstraction Layer for memory operations
pub struct MemoryHAL {
    heap_base: *mut u8,
    heap_size: usize,
}

impl MemoryHAL {
    pub fn new(size: usize) -> Self {
        // In a real system, we'd allocate this properly
        // For now, we're using a Vec as our heap
        let mut heap = Vec::with_capacity(size);
        let ptr = heap.as_mut_ptr();
        std::mem::forget(heap); // Don't drop the vec, we'll manage this memory manually

        Self {
            heap_base: ptr,
            heap_size: size,
        }
    }

    pub fn read_byte(&self, addr: usize) -> Option<u8> {
        if addr >= self.heap_size {
            return None;
        }

        unsafe {
            Some(*self.heap_base.add(addr))
        }
    }

    pub fn write_byte(&mut self, addr: usize, value: u8) -> Result<(), &'static str> {
        if addr >= self.heap_size {
            return Err("Memory access out of bounds");
        }

        unsafe {
            *self.heap_base.add(addr) = value;
        }

        Ok(())
    }

    pub fn get_heap_size(&self) -> usize {
        self.heap_size
    }
}

impl Drop for MemoryHAL {
    fn drop(&mut self) {
        unsafe {
            Vec::from_raw_parts(self.heap_base, 0, self.heap_size);
        }
    }
}