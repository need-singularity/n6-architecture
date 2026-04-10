use std::sync::Mutex;

/// Simple buffer pool for reusing Vec<f32> allocations.
/// Avoids repeated heap allocation in hot loops.
pub struct BufferPool {
    pool: Mutex<Vec<Vec<f32>>>,
    default_capacity: usize,
}

impl BufferPool {
    /// Create a new pool. `default_capacity` is the initial Vec capacity for new buffers.
    pub fn new(default_capacity: usize) -> Self {
        Self {
            pool: Mutex::new(Vec::new()),
            default_capacity,
        }
    }

    /// Get a buffer from the pool, or allocate a new one.
    /// The returned buffer is cleared (len=0) but retains its capacity.
    pub fn get(&self) -> Vec<f32> {
        let mut pool = self.pool.lock().unwrap();
        match pool.pop() {
            Some(mut buf) => {
                buf.clear();
                buf
            }
            None => Vec::with_capacity(self.default_capacity),
        }
    }

    /// Return a buffer to the pool for reuse.
    pub fn put(&self, buf: Vec<f32>) {
        let mut pool = self.pool.lock().unwrap();
        pool.push(buf);
    }

    /// Number of buffers currently in the pool.
    pub fn available(&self) -> usize {
        self.pool.lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_reuse() {
        let pool = BufferPool::new(1024);
        assert_eq!(pool.available(), 0);

        let mut buf = pool.get();
        buf.extend_from_slice(&[1.0, 2.0, 3.0]);
        assert!(buf.capacity() >= 1024);

        pool.put(buf);
        assert_eq!(pool.available(), 1);

        let buf2 = pool.get();
        assert_eq!(buf2.len(), 0); // cleared
        assert!(buf2.capacity() >= 1024); // capacity retained
        assert_eq!(pool.available(), 0);
    }

    #[test]
    fn pool_zero_capacity() {
        let pool = BufferPool::new(0);
        let buf = pool.get();
        assert_eq!(buf.len(), 0);
        pool.put(buf);
        assert_eq!(pool.available(), 1);

        let buf2 = pool.get();
        assert_eq!(buf2.len(), 0);
        assert_eq!(pool.available(), 0);
    }

    #[test]
    fn pool_multiple_buffers() {
        let pool = BufferPool::new(64);

        // Get and return several buffers
        let mut bufs: Vec<Vec<f32>> = (0..5).map(|_| pool.get()).collect();
        assert_eq!(pool.available(), 0);

        for buf in &mut bufs {
            buf.push(42.0);
        }

        for buf in bufs {
            pool.put(buf);
        }
        assert_eq!(pool.available(), 5);

        // Drain all — each should be cleared
        for _ in 0..5 {
            let b = pool.get();
            assert_eq!(b.len(), 0);
        }
        assert_eq!(pool.available(), 0);
    }

    #[test]
    fn pool_large_capacity() {
        let pool = BufferPool::new(1_000_000);
        let buf = pool.get();
        assert!(buf.capacity() >= 1_000_000);
        pool.put(buf);

        let buf2 = pool.get();
        assert!(buf2.capacity() >= 1_000_000);
    }
}
