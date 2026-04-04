/// Memory Allocators — n/phi=3 files
///
/// egyptian: runtime memory allocator (1/2 + 1/3 + 1/6 = 1 heap partition)
/// arena:    compiler-internal arena allocator for IR nodes

pub mod egyptian;
pub mod arena;
