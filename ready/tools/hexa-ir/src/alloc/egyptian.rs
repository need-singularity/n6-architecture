/// Egyptian Allocator — 1/2 + 1/3 + 1/6 = 1 memory allocation
///
/// Splits heap into 3 regions sized by Egyptian fractions of 6:
///   Region A: 1/2 of total (large objects, block = 2^sigma = 4096)
///   Region B: 1/3 of total (medium objects, block = 2^(sigma-phi) = 1024)
///   Region C: 1/6 of total (small objects, block = 2^(sigma-tau) = 256)
///
/// External fragmentation = 0 by design (fixed-size blocks per region).

use crate::util::n6::*;
use std::collections::VecDeque;

/// Region pool with fixed-size blocks
struct RegionPool {
    block_size: usize,
    total_blocks: usize,
    free_blocks: VecDeque<usize>,
    allocated: Vec<bool>,
}

impl RegionPool {
    fn new(region_bytes: usize, block_size: usize) -> Self {
        let total_blocks = region_bytes / block_size;
        let free_blocks: VecDeque<usize> = (0..total_blocks).collect();
        let allocated = vec![false; total_blocks];
        RegionPool { block_size, total_blocks, free_blocks, allocated }
    }

    fn alloc(&mut self) -> Option<usize> {
        if let Some(idx) = self.free_blocks.pop_front() {
            self.allocated[idx] = true;
            Some(idx)
        } else {
            None
        }
    }

    fn free(&mut self, idx: usize) {
        if idx < self.total_blocks && self.allocated[idx] {
            self.allocated[idx] = false;
            self.free_blocks.push_back(idx);
        }
    }

    fn utilization(&self) -> f64 {
        if self.total_blocks == 0 { return 0.0; }
        let used = self.allocated.iter().filter(|&&x| x).count();
        used as f64 / self.total_blocks as f64
    }
}

/// Egyptian fraction heap allocator
pub struct EgyptianAllocator {
    /// Region A: 1/2 of heap, block_size = 2^sigma = 4096
    region_a: RegionPool,
    /// Region B: 1/3 of heap, block_size = 2^(sigma-phi) = 1024
    region_b: RegionPool,
    /// Region C: 1/6 of heap, block_size = 2^(sigma-tau) = 256
    region_c: RegionPool,
    total_bytes: usize,
}

/// Allocation handle: (region_tag, block_index)
#[derive(Clone, Copy, Debug)]
pub struct AllocHandle {
    pub region: char,
    pub index: usize,
}

impl EgyptianAllocator {
    /// Create a new allocator with the given total heap size
    pub fn new(total_bytes: usize) -> Self {
        // 1/2 + 1/3 + 1/6 = 1 (exact partition, no waste)
        let region_a_bytes = total_bytes / 2;  // 1/2
        let region_b_bytes = total_bytes / 3;  // 1/3
        let region_c_bytes = total_bytes / N;  // 1/6

        EgyptianAllocator {
            region_a: RegionPool::new(region_a_bytes, BLOCK_LARGE),
            region_b: RegionPool::new(region_b_bytes, BLOCK_MEDIUM),
            region_c: RegionPool::new(region_c_bytes, BLOCK_SMALL),
            total_bytes,
        }
    }

    /// Allocate a block of at least `size` bytes
    pub fn alloc(&mut self, size: usize) -> Option<AllocHandle> {
        if size > BLOCK_MEDIUM {
            self.region_a.alloc().map(|i| AllocHandle { region: 'A', index: i })
        } else if size > BLOCK_SMALL {
            self.region_b.alloc().map(|i| AllocHandle { region: 'B', index: i })
        } else {
            self.region_c.alloc().map(|i| AllocHandle { region: 'C', index: i })
        }
    }

    /// Free a previously allocated block
    pub fn free(&mut self, handle: AllocHandle) {
        match handle.region {
            'A' => self.region_a.free(handle.index),
            'B' => self.region_b.free(handle.index),
            _ => self.region_c.free(handle.index),
        }
    }

    /// External fragmentation is always 0 (fixed-size blocks per region)
    pub fn external_fragmentation(&self) -> f64 {
        0.0
    }

    /// Overall heap utilization
    pub fn utilization(&self) -> f64 {
        let total = self.region_a.total_blocks + self.region_b.total_blocks
                  + self.region_c.total_blocks;
        if total == 0 { return 0.0; }
        let used = self.region_a.allocated.iter().filter(|&&x| x).count()
                 + self.region_b.allocated.iter().filter(|&&x| x).count()
                 + self.region_c.allocated.iter().filter(|&&x| x).count();
        used as f64 / total as f64
    }

    /// Total heap size in bytes
    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }
}
