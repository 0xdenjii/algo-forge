// Lock-free - Single-Producer Single-Consumer Ring Buffer
// Implement a bounded wait-free queue for exactly one producer and one consumer
// thread, with no locks, using atomics and correct memory ordering.
// Approach: fixed array of MaybeUninit slots; head (consumer) and tail (producer)
// are monotonically increasing AtomicUsize indices. Producer publishes with Release
// on tail, consumer reads tail with Acquire (and symmetrically for head).
// Time: O(1) push/pop. Space: O(capacity).
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub struct Spsc<T> {
    buf: Box<[UnsafeCell<MaybeUninit<T>>]>,
    head: AtomicUsize,
    tail: AtomicUsize,
}
unsafe impl<T: Send> Sync for Spsc<T> {}
unsafe impl<T: Send> Send for Spsc<T> {}

impl<T> Spsc<T> {
    pub fn new(cap: usize) -> Self {
        assert!(cap > 0);
        let buf = (0..cap).map(|_| UnsafeCell::new(MaybeUninit::uninit())).collect();
        Self { buf, head: AtomicUsize::new(0), tail: AtomicUsize::new(0) }
    }
    /// Must only be called from the single producer thread.
    pub fn push(&self, v: T) -> Result<(), T> {
        let tail = self.tail.load(Ordering::Relaxed);
        if tail - self.head.load(Ordering::Acquire) == self.buf.len() {
            return Err(v);
        }
        unsafe { (*self.buf[tail % self.buf.len()].get()).write(v) };
        self.tail.store(tail + 1, Ordering::Release);
        Ok(())
    }
    /// Must only be called from the single consumer thread.
    pub fn pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        if head == self.tail.load(Ordering::Acquire) {
            return None;
        }
        let v = unsafe { (*self.buf[head % self.buf.len()].get()).assume_init_read() };
        self.head.store(head + 1, Ordering::Release);
        Some(v)
    }
}

impl<T> Drop for Spsc<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

fn main() {
    const N: u64 = 1_000_000;
    let q = Arc::new(Spsc::new(1024));
    let p = q.clone();
    let producer = thread::spawn(move || {
        for i in 0..N {
            let mut v = i;
            while let Err(back) = p.push(v) {
                v = back;
                std::hint::spin_loop();
            }
        }
    });
    let mut expected = 0;
    while expected < N {
        if let Some(v) = q.pop() {
            assert_eq!(v, expected); // FIFO order preserved
            expected += 1;
        } else {
            std::hint::spin_loop();
        }
    }
    producer.join().unwrap();
    println!("transferred {N} items in order");
}
