/// Arena Allocator — fast bump allocation for compiler-internal use
///
/// The arena allocates objects in contiguous chunks, with O(1) allocation
/// and bulk deallocation (drop the entire arena at once).
/// Used for AST nodes, IR instructions, and temporary data during compilation.
///
/// Chunk size: 2^sigma = 4096 bytes (matching page size and BLOCK_LARGE).

use crate::util::n6::BLOCK_LARGE;

/// A single contiguous memory chunk
struct Chunk {
    data: Vec<u8>,
    used: usize,
}

impl Chunk {
    fn new(capacity: usize) -> Self {
        Chunk {
            data: vec![0u8; capacity],
            used: 0,
        }
    }

    fn alloc(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        // Align up
        let aligned = (self.used + align - 1) & !(align - 1);
        if aligned + size > self.data.len() {
            return None;
        }
        let ptr = unsafe { self.data.as_mut_ptr().add(aligned) };
        self.used = aligned + size;
        Some(ptr)
    }

    fn remaining(&self) -> usize {
        self.data.len() - self.used
    }
}

/// Arena allocator with automatic chunk growth
pub struct Arena {
    chunks: Vec<Chunk>,
    chunk_size: usize,
    total_allocated: usize,
}

impl Arena {
    /// Create a new arena with default chunk size (2^sigma = 4096)
    pub fn new() -> Self {
        Arena {
            chunks: vec![Chunk::new(BLOCK_LARGE)],
            chunk_size: BLOCK_LARGE,
            total_allocated: 0,
        }
    }

    /// Create an arena with a custom chunk size
    pub fn with_chunk_size(size: usize) -> Self {
        let size = size.max(64); // minimum 2^n = 64 bytes
        Arena {
            chunks: vec![Chunk::new(size)],
            chunk_size: size,
            total_allocated: 0,
        }
    }

    /// Allocate `size` bytes with `align` alignment
    pub fn alloc_raw(&mut self, size: usize, align: usize) -> *mut u8 {
        // Try current chunk first
        if let Some(chunk) = self.chunks.last_mut() {
            if let Some(ptr) = chunk.alloc(size, align) {
                self.total_allocated += size;
                return ptr;
            }
        }

        // Allocate a new chunk (at least chunk_size or size, whichever is larger)
        let new_size = self.chunk_size.max(size + align);
        let mut chunk = Chunk::new(new_size);
        let ptr = chunk.alloc(size, align).expect("fresh chunk should have space");
        self.chunks.push(chunk);
        self.total_allocated += size;
        ptr
    }

    /// Allocate and zero-initialize space for a value of type T
    pub fn alloc_zeroed<T: Sized>(&mut self) -> &mut T {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        let ptr = self.alloc_raw(size, align) as *mut T;
        unsafe {
            std::ptr::write_bytes(ptr, 0, 1);
            &mut *ptr
        }
    }

    /// Total bytes allocated through this arena
    pub fn total_allocated(&self) -> usize {
        self.total_allocated
    }

    /// Total bytes reserved (sum of all chunk capacities)
    pub fn total_reserved(&self) -> usize {
        self.chunks.iter().map(|c| c.data.len()).sum()
    }

    /// Number of chunks allocated
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    /// Reset the arena: reuse chunks but mark them all as empty
    pub fn reset(&mut self) {
        for chunk in &mut self.chunks {
            chunk.used = 0;
        }
        self.total_allocated = 0;
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::n6::*;

    #[test]
    fn test_arena_basic_alloc() {
        let alloc_size = PHI_N;  // n6: 2^n=64
        let mut arena = Arena::new();
        let ptr = arena.alloc_raw(alloc_size, SIGMA_TAU);  // n6: align=σ-τ=8
        assert!(!ptr.is_null());
        assert_eq!(arena.total_allocated(), alloc_size);
    }

    #[test]
    fn test_arena_growth() {
        let chunk_size = PHI_N * PHI;  // n6: 2^n·φ=128
        let mut arena = Arena::with_chunk_size(chunk_size);
        // Allocate more than one chunk — 10=σ-φ iterations
        for _ in 0..SIGMA_PHI {
            arena.alloc_raw(PHI_N, SIGMA_TAU);  // n6: 2^n=64, align=σ-τ=8
        }
        assert!(arena.chunk_count() > 1);
        assert_eq!(arena.total_allocated(), PHI_N * SIGMA_PHI);  // n6: 64·10=640
    }

    #[test]
    fn test_arena_reset() {
        let mut arena = Arena::new();
        let alloc_a = SIGMA_PHI * SIGMA_PHI;  // n6: (σ-φ)²=100
        let alloc_b = PHI * alloc_a;           // n6: φ·(σ-φ)²=200
        arena.alloc_raw(alloc_a, SIGMA_TAU);
        arena.alloc_raw(alloc_b, SIGMA_TAU);
        let chunks_before = arena.chunk_count();
        arena.reset();
        assert_eq!(arena.total_allocated(), 0);
        assert_eq!(arena.chunk_count(), chunks_before); // chunks preserved
    }
}
